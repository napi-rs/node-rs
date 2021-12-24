#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use std::str::FromStr;

use napi::bindgen_prelude::*;
use napi::{Error, JsBuffer, Result, Status};
use napi_derive::*;

use crate::hash_task::AsyncHashInput;
use crate::hash_task::HashTask;
use crate::lib_bcrypt::{format_salt, gen_salt, Version};
use crate::verify_task::VerifyTask;

mod b64;
mod errors;
mod hash_task;
mod lib_bcrypt;
mod salt_task;
mod verify_task;

#[napi]
pub const DEFAULT_COST: u32 = 12;

#[napi]
pub fn gen_salt_sync(round: u32, version: String) -> Result<String> {
  let salt = gen_salt();
  Ok(format_salt(
    round,
    Version::from_str(version.as_str()).map_err(|_| Error::from_status(Status::InvalidArg))?,
    &salt,
  ))
}

#[napi(js_name = "genSalt")]
pub fn gen_salt_js(
  round: u32,
  version: String,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<salt_task::SaltTask>> {
  let task = salt_task::SaltTask {
    round,
    version: Version::from_str(version.as_str())
      .map_err(|_| Error::from_status(Status::InvalidArg))?,
  };
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn hash_sync(input: Either<String, Buffer>, cost: Option<u32>) -> Result<String> {
  match input {
    Either::A(s) => HashTask::hash(s.as_bytes(), cost.unwrap_or(DEFAULT_COST)),
    Either::B(b) => HashTask::hash(b.as_ref(), cost.unwrap_or(DEFAULT_COST)),
  }
}

#[napi]
pub fn hash(
  input: Either<String, JsBuffer>,
  cost: Option<u32>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<HashTask>> {
  let task = HashTask::new(
    AsyncHashInput::from_either(input)?,
    cost.unwrap_or(DEFAULT_COST),
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
