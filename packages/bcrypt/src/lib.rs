#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;
use std::str::FromStr;

use napi::{CallContext, Error, JsBoolean, JsBuffer, JsNumber, JsObject, JsString, Result, Status};

use crate::hash_task::HashTask;
use crate::lib_bcrypt::{format_salt, gen_salt, Version};
use crate::verify_task::VerifyTask;

mod b64;
mod bcrypt;
mod errors;
mod hash_task;
mod lib_bcrypt;
mod salt_task;
mod verify_task;

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_os = "freebsd"),
  not(target_arch = "arm"),
  not(target_arch = "aarch64")
))]
#[cfg(all(windows, not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("genSaltSync", js_salt)?;
  exports.create_named_method("genSalt", js_async_salt)?;
  exports.create_named_method("hash", js_async_hash)?;
  exports.create_named_method("hashSync", js_hash)?;
  exports.create_named_method("verifySync", js_verify)?;
  exports.create_named_method("verify", js_async_verify)?;

  Ok(())
}

#[js_function(2)]
fn js_salt(ctx: CallContext) -> Result<JsString> {
  let round = ctx.get::<JsNumber>(0)?;
  let version = ctx.get::<JsString>(1)?.into_utf8()?;
  let salt = gen_salt();
  let salt_string = format_salt(
    round.try_into()?,
    Version::from_str(version.as_str()?).map_err(|_| Error::from_status(Status::InvalidArg))?,
    &salt,
  );
  ctx.env.create_string(&salt_string)
}

#[js_function(2)]
fn js_async_salt(ctx: CallContext) -> Result<JsObject> {
  let round = ctx.get::<JsNumber>(0)?;
  let version = ctx.get::<JsString>(1)?.into_utf8()?;
  let task = salt_task::SaltTask {
    round: round.try_into()?,
    version: Version::from_str(version.as_str()?)
      .map_err(|_| Error::from_status(Status::InvalidArg))?,
  };
  ctx.env.spawn(task).map(|t| t.promise_object())
}

#[js_function(2)]
fn js_hash(ctx: CallContext) -> Result<JsString> {
  let password = ctx.get::<JsBuffer>(0)?.into_value()?;
  let cost = ctx.get::<JsNumber>(1)?;
  let result = HashTask::hash(&password, cost.try_into()?)?;
  ctx.env.create_string(result.as_str())
}

#[js_function(2)]
fn js_async_hash(ctx: CallContext) -> Result<JsObject> {
  let password = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let cost = ctx.get::<JsNumber>(1)?;
  let task = HashTask::new(password, cost.try_into()?);
  ctx.env.spawn(task).map(|t| t.promise_object())
}

#[js_function(2)]
fn js_verify(ctx: CallContext) -> Result<JsBoolean> {
  let password = ctx.get::<JsBuffer>(0)?.into_value()?;
  let hash = ctx.get::<JsBuffer>(1)?.into_value()?;
  let result = VerifyTask::verify(&password, &hash)?;
  ctx.env.get_boolean(result)
}

#[js_function(2)]
fn js_async_verify(ctx: CallContext) -> Result<JsObject> {
  let password = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let hash = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = VerifyTask::new(password, hash);
  ctx.env.spawn(task).map(|t| t.promise_object())
}
