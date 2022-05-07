#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use hash_task::HashOptions;
use napi::bindgen_prelude::*;
use napi::{Error, JsBuffer, Result, Status};
use napi_derive::*;

use crate::hash_task::AsyncHashInput;
use crate::hash_task::HashTask;
use crate::salt_task::gen_salt;
use crate::verify_task::VerifyTask;

mod hash_task;
mod salt_task;
mod verify_task;

#[napi]
pub const DEFAULT_COST: u32 = 12;

#[napi]
pub fn gen_salt_sync() -> Result<Buffer> {
  let salt = gen_salt().map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Generate salt failed {}", err),
    )
  })?;
  Ok(salt.to_vec().into())
}

#[napi(js_name = "genSalt")]
pub fn gen_salt_js(signal: Option<AbortSignal>) -> Result<AsyncTask<salt_task::SaltTask>> {
  let task = salt_task::SaltTask {};
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn hash_sync(input: Either<String, JsBuffer>, options: Option<HashOptions>) -> Result<String> {
  let mut task = HashTask::new(
    AsyncHashInput::from_either(input)?,
    options.unwrap_or_default(),
  );
  task.compute()
}

#[napi]
pub fn hash(
  input: Either<String, JsBuffer>,
  options: Option<HashOptions>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<HashTask>> {
  let task = HashTask::new(
    AsyncHashInput::from_either(input)?,
    options.unwrap_or_default(),
  );
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn verify_sync(input: Either<String, Buffer>, hash: Either<String, Buffer>) -> Result<bool> {
  let input = either_string_buffer_as_bytes(&input);
  let hash = either_string_buffer_as_bytes(&hash);
  VerifyTask::verify(input, hash)
}

#[inline(always)]
fn either_string_buffer_as_bytes(input: &Either<String, Buffer>) -> &[u8] {
  match input {
    Either::A(s) => s.as_bytes(),
    Either::B(b) => b.as_ref(),
  }
}

#[napi]
pub fn verify(
  password: Either<String, JsBuffer>,
  hash: Either<String, JsBuffer>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  let task = VerifyTask::new(
    AsyncHashInput::from_either(password)?,
    AsyncHashInput::from_either(hash)?,
  );
  Ok(AsyncTask::with_optional_signal(task, signal))
}
