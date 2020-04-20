#[macro_use]
extern crate napi_rs as napi;

use crc32fast::Hasher;
use napi::{Any, Buffer, Env, Error, Number, Object, Result, Status, Value};

register_module!(test_module, init);

fn init<'env>(
  env: &'env Env,
  exports: &'env mut Value<'env, Object>,
) -> Result<Option<Value<'env, Object>>> {
  exports.set_named_property(
    "calculate",
    env.create_function("calculate", callback!(calculate))?,
  )?;
  Ok(None)
}

fn calculate<'a>(
  env: &'a Env,
  _this: Value<'a, Any>,
  args: &[Value<'a, Any>],
) -> Result<Option<Value<'a, Number>>> {
  let input_data = args
    .get(0)
    .map(|v| Value::<Buffer>::from_raw(env, v.into_raw()))
    .ok_or(Error::new(Status::InvalidArg))??;
  let mut hasher = Hasher::new();
  hasher.update(&input_data);
  let output = env.create_uint32(hasher.finalize())?;
  Ok(Some(output))
}
