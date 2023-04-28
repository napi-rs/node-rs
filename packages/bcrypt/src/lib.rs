#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use bcrypt::Version;
use napi::bindgen_prelude::*;
use napi::{Error, JsBuffer, Result, Status};
use napi_derive::*;

use crate::hash_task::AsyncHashInput;
use crate::hash_task::HashTask;
use crate::salt_task::{format_salt, gen_salt};
use crate::verify_task::VerifyTask;

mod hash_task;
mod salt_task;
mod verify_task;

#[napi]
pub const DEFAULT_COST: u32 = 12;

#[napi]
pub fn gen_salt_sync(round: u32, version: Option<String>) -> Result<String> {
  let salt = gen_salt().map_err(|err| {
    Error::new(
      Status::GenericFailure,
      format!("Generate salt failed {err}"),
    )
  })?;
  Ok(format_salt(round, &version_from_str(version)?, &salt))
}

#[napi(js_name = "genSalt")]
pub fn gen_salt_js(
  round: u32,
  version: Option<String>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<salt_task::SaltTask>> {
  let task = salt_task::SaltTask {
    round,
    version: version_from_str(version)?,
  };
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn hash_sync(
  input: Either<String, Buffer>,
  cost: Option<u32>,
  salt: Option<Buffer>,
) -> Result<String> {
  let salt = if let Some(salt) = salt {
    let mut s = [0u8; 16];
    s.copy_from_slice(salt.as_ref());
    s
  } else {
    gen_salt().map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))?
  };
  match input {
    Either::A(s) => HashTask::hash(s.as_bytes(), salt, cost.unwrap_or(DEFAULT_COST)),
    Either::B(b) => HashTask::hash(b.as_ref(), salt, cost.unwrap_or(DEFAULT_COST)),
  }
}

#[napi]
pub fn hash(
  input: Either<String, JsBuffer>,
  cost: Option<u32>,
  salt: Option<Buffer>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<HashTask>> {
  let salt = if let Some(salt) = salt {
    let mut s = [0u8; 16];
    s.copy_from_slice(salt.as_ref());
    s
  } else {
    gen_salt().map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))?
  };
  let task = HashTask::new(
    AsyncHashInput::from_either(input)?,
    cost.unwrap_or(DEFAULT_COST),
    salt,
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

#[inline]
fn version_from_str(version: Option<String>) -> Result<Version> {
  match version.as_deref() {
    Some("2a") => Ok(Version::TwoA),
    Some("2b") | None => Ok(Version::TwoB),
    Some("2y") => Ok(Version::TwoX),
    Some("2x") => Ok(Version::TwoY),
    Some(version) => Err(Error::new(
      Status::InvalidArg,
      format!("{version} is not a valid version"),
    )),
  }
}
