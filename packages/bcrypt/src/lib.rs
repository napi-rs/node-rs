#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use crate::lib_bcrypt::{format_salt, gen_salt, Version};
use hash_task::HashTask;
use napi::{
  Boolean, Buffer, CallContext, Env, Error, JsString, Number, Object, Result, Status, Value,
};
use std::convert::TryInto;
use std::str::FromStr;
use verify_task::VerifyTask;

mod hash_task;
mod verify_task;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod b64;
mod bcrypt;
mod errors;
mod lib_bcrypt;

#[cfg(not(test))]
register_module!(test_module, init);

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_property(
    env.create_string("hash")?,
    env.create_function("hash", js_async_hash)?,
  )?;

  exports.set_property(
    env.create_string("hashSync")?,
    env.create_function("hashSync", js_hash)?,
  )?;

  exports.set_property(
    env.create_string("genSalt")?,
    env.create_function("genSalt", js_salt)?,
  )?;

  exports.set_property(
    env.create_string("verifySync")?,
    env.create_function("verifySync", js_verify)?,
  )?;

  exports.set_property(
    env.create_string("verify")?,
    env.create_function("verify", js_async_verify)?,
  )?;

  Ok(())
}

#[js_function(2)]
fn js_salt(ctx: CallContext) -> Result<Value<JsString>> {
  let round = ctx.get::<Number>(0)?;
  let version = ctx.get::<JsString>(1)?;
  let salt = gen_salt();
  let salt_string = format_salt(
    round.try_into()?,
    Version::from_str(version.as_str()?).map_err(|_| Error::from_status(Status::InvalidArg))?,
    &salt,
  );
  ctx.env.create_string(&salt_string)
}

#[js_function(2)]
fn js_hash(ctx: CallContext) -> Result<Value<JsString>> {
  let password = ctx.get::<Buffer>(0)?;
  let cost = ctx.get::<Number>(1)?;
  let result = HashTask::hash(password, cost.try_into()?)?;
  ctx.env.create_string(result.as_str())
}

#[js_function(2)]
fn js_async_hash(ctx: CallContext) -> Result<Value<Object>> {
  let password = ctx.get::<Buffer>(0)?;
  let cost = ctx.get::<Number>(1)?;
  let task = HashTask::new(password, cost.try_into()?);
  ctx.env.spawn(task)
}

#[js_function(2)]
fn js_verify(ctx: CallContext) -> Result<Value<Boolean>> {
  let password = ctx.get::<Buffer>(0)?;
  let hash = ctx.get::<Buffer>(1)?;
  let result =
    VerifyTask::verify(password, hash).map_err(|_| Error::from_status(Status::GenericFailure))?;
  ctx.env.get_boolean(result)
}

#[js_function(2)]
fn js_async_verify(ctx: CallContext) -> Result<Value<Object>> {
  let password = ctx.get::<Buffer>(0)?;
  let hash = ctx.get::<Buffer>(1)?;
  let task = VerifyTask::new(password, hash);
  ctx.env.spawn(task)
}
