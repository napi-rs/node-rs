#[macro_use]
extern crate napi_rs as napi;

use futures::io::{copy, AllowStdIo};
use image::png::PngDecoder;
use image::ImageDecoder;
use napi::{Any, Buffer, Env, Error, Object, Result, Status, Value};
use std::ops::Deref;

register_module!(node_rs_png, init);

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {
  exports.set_named_property("decode", env.create_function("decode", callback!(decode)))?;
  Ok(None)
}

fn decode<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let async_context = env.async_init(None, "test_spawn");
  let png_data: Value<Buffer> = args
    .get(0)
    .map(|v| Value::<Buffer>::from_value(env, v))
    .ok_or_else(|| Error::new(Status::InvalidArg))?;

  let image_data = {
    let data = png_data.deref();
    unsafe { std::slice::from_raw_parts(data.as_ptr(), data.len()) }
  };

  let decoder = PngDecoder::new(image_data).map_err(|e| {
    dbg!("{:?}", e);
    Error::new(Status::GenericFailure)
  })?;
  let total_bytes = decoder.total_bytes() as usize;
  let async_reader = AllowStdIo::new(decoder.into_reader().map_err(|e| {
    dbg!("{:?}", e);
    Error::new(Status::GenericFailure)
  })?);

  let (promise, deferred) = env.create_promise();
  let task = async move {
    let mut output = vec![0; total_bytes];
    match copy(async_reader, &mut output).await {
      Ok(_) => {
        async_context.enter(move |env| {
          env.resolve_deferred(deferred, env.create_buffer_with_data(output));
        });
      }
      Err(e) => {
        dbg!("{:?}", &e);
        async_context.enter(move |env| {
          env.reject_deferred(
            deferred,
            env.create_error(Status::GenericFailure, &format!("{:?}", e)),
          );
        });
      }
    }
  };

  env.create_executor().execute(task);

  Ok(Some(promise.try_into().unwrap()))
}
