#[macro_use]
extern crate napi_rs as napi;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::convert::TryInto;
use std::ops::DerefMut;

use serde_json::Value as SerdeValue;
use simd_json;

use napi::{Any, Buffer, Env, Error, Object, Result, Status, String as JsString, Value};

register_module!(SIMD_JSON, init);

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {
  exports.set_named_property("parse", env.create_function("parse", callback!(parse)))?;
  exports.set_named_property(
    "parseString",
    env.create_function("parseString", callback!(parse_string)),
  )?;
  Ok(None)
}

fn parse_string<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let mut d = args
    .get(0)
    .map(|v| Value::<JsString>::from_raw(env, v.into_raw()))
    .ok_or(Error::new(Status::InvalidArg))?;
  let v: SerdeValue =
    simd_json::serde::from_slice(d.deref_mut()).map_err(|_| Error::new(Status::InvalidArg))?;
  env.get_undefined().map(|v| Some(v.into_any()))
  // env
  //   .from_serde_value(Box::leak(Box::new(v)))
  //   .map(|v| Some(v))
}

fn parse<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let mut input_buf = args
    .get(0)
    .map(|v| Value::<Buffer>::from_raw(env, v.into_raw()))
    .ok_or(Error::new(Status::InvalidArg))?;
  let input_buf_slice = input_buf.deref_mut();
  let v: SerdeValue = simd_json::to_borrowed_value(input_buf_slice)
    .map(|v| v.try_into().unwrap())
    .map_err(|e| {
      dbg!(e);
      Error::new(Status::InvalidArg)
    })?;
  // env.get_undefined().map(|v| Some(v.into_any()))
  env
    .from_serde_value(Box::leak(Box::new(v)))
    .map(|v| Some(v))
}
