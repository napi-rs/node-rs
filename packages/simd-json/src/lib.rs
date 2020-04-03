#[macro_use]
extern crate napi_rs as napi;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use simd_json;
use serde_json::Value as SerdeValue;

use napi::{Any, Env, Error, Object, Result, Status, Value, Buffer};

register_module!(SIMD_JSON, init);

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {
  exports.set_named_property("parse", env.create_function("parse", callback!(parse))?)?;
  Ok(None)
}

fn parse<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Any>>> {
  let d = args
    .get(0)
    .map(|v| Value::<Buffer>::from_value(env, v))
    .ok_or(Error::new(Status::InvalidArg))?;
  let v: SerdeValue = simd_json::serde::from_slice(d?.get_ref_mut()?).map_err(|_| Error::new(Status::InvalidArg))?;
  env.from_serde_value(Box::leak(Box::new(v))).map(|v| Some(v))
}
