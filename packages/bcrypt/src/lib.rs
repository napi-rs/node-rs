#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use crate::lib_bcrypt::{format_salt, gen_salt, Version};
use hash_task::HashTask;
use napi::{
  CallContext, Error, JsBoolean, JsBuffer, JsNumber, JsObject, JsString, Module, Result, Status,
};
use std::convert::TryInto;
use std::str::FromStr;
use verify_task::VerifyTask;

mod hash_task;
mod verify_task;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod b64;
mod bcrypt;
mod errors;
mod lib_bcrypt;

#[cfg(not(test))]
register_module!(bcrypt, init);

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("hash", js_async_hash)?;

  module.create_named_method("hashSync", js_hash)?;

  module.create_named_method("genSalt", js_salt)?;

  module.create_named_method("verifySync", js_verify)?;

  module.create_named_method("verify", js_async_verify)?;

  Ok(())
}

#[js_function(2)]
fn js_salt(ctx: CallContext) -> Result<JsString> {
  let round = ctx.get::<JsNumber>(0)?;
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
fn js_hash(ctx: CallContext) -> Result<JsString> {
  let password = ctx.get::<JsBuffer>(0)?;
  let cost = ctx.get::<JsNumber>(1)?;
  let result = HashTask::hash(&password, cost.try_into()?)?;
  ctx.env.create_string(result.as_str())
}

#[js_function(2)]
fn js_async_hash(ctx: CallContext) -> Result<JsObject> {
  let password = ctx.get::<JsBuffer>(0)?;
  let cost = ctx.get::<JsNumber>(1)?;
  let task = HashTask::new(password, cost.try_into()?);
  ctx.env.spawn(task)
}

#[js_function(2)]
fn js_verify(ctx: CallContext) -> Result<JsBoolean> {
  let password = ctx.get::<JsBuffer>(0)?;
  let hash = ctx.get::<JsBuffer>(1)?;
  let result = VerifyTask::verify(&password, &hash)
    .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))?;
  ctx.env.get_boolean(result)
}

#[js_function(2)]
fn js_async_verify(ctx: CallContext) -> Result<JsObject> {
  let password = ctx.get::<JsBuffer>(0)?;
  let hash = ctx.get::<JsBuffer>(1)?;
  let task = VerifyTask::new(password, hash);
  ctx.env.spawn(task)
}
