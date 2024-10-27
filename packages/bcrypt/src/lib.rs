#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use std::cmp;

use bcrypt::Version;
use napi::bindgen_prelude::*;
use napi_derive::*;

use crate::hash_task::HashTask;
use crate::salt_task::{format_salt, gen_salt};
use crate::verify_task::VerifyTask;

mod hash_task;
mod salt_task;
mod verify_task;

#[napi]
pub const DEFAULT_COST: u32 = 12;

#[napi(ts_args_type = "round: number, version?: '2a' | '2x' | '2y' | '2b'")]
pub fn gen_salt_sync(round: u32, version: Option<String>) -> Result<String> {
  let salt = gen_salt();
  Ok(format_salt(round, &version_from_str(version)?, &salt))
}

#[napi(
  js_name = "genSalt",
  ts_args_type = "round: number, version?: '2a' | '2x' | '2y' | '2b', signal?: AbortSignal"
)]
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
#[inline]
pub fn hash_sync(
  input: Either<String, &[u8]>,
  cost: Option<u32>,
  salt: Option<Either<String, &[u8]>>,
) -> Result<String> {
  let salt = if let Some(salt) = salt {
    let mut s = [0u8; 16];
    let buf = salt.as_ref();
    // make sure salt buffer length should be 16
    let copy_length = cmp::min(buf.len(), s.len());
    s[..copy_length].copy_from_slice(&buf[..copy_length]);
    s
  } else {
    rand::random()
  };
  HashTask::hash(input.as_ref(), salt, cost.unwrap_or(DEFAULT_COST))
}

#[napi]
pub fn hash(
  input: Either<Uint8Array, String>,
  cost: Option<u32>,
  salt: Option<Either<String, &[u8]>>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<HashTask>> {
  let salt = if let Some(salt) = salt {
    let mut s = [0u8; 16];
    let buf = salt.as_ref();
    // make sure salt buffer length should be 16
    let copy_length = cmp::min(buf.len(), s.len());
    s[..copy_length].copy_from_slice(&buf[..copy_length]);
    s
  } else {
    gen_salt()
  };
  let task = HashTask::new(input, cost.unwrap_or(DEFAULT_COST), salt);
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
#[inline]
pub fn verify_sync(input: Either<String, &[u8]>, hash: Either<String, &[u8]>) -> Result<bool> {
  VerifyTask::verify(input, hash)
}

#[napi]
pub fn verify(
  password: Either<Uint8Array, String>,
  hash: Either<Uint8Array, String>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  let task = VerifyTask::new(password, hash);
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[inline]
fn version_from_str(version: Option<String>) -> Result<Version> {
  match version.as_deref() {
    Some("2a") => Ok(Version::TwoA),
    Some("2b") | None => Ok(Version::TwoB),
    Some("2x") => Ok(Version::TwoX),
    Some("2y") => Ok(Version::TwoY),
    Some(version) => Err(Error::new(
      Status::InvalidArg,
      format!("{version} is not a valid version"),
    )),
  }
}
