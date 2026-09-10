#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use napi::bindgen_prelude::*;
use napi_derive::*;

use crate::hash_task::HashTask;
use crate::options::{hash_options, validate_cost, version_from_str};
use crate::salt_task::{format_salt, gen_salt};
use crate::verify_task::VerifyTask;

mod hash_task;
mod options;
mod salt_task;
mod verify_task;

#[napi]
pub const DEFAULT_COST: u32 = 12;

/// Internal binding contract, checked by the public JavaScript wrapper.
#[napi]
pub const BCRYPT_API_VERSION: u32 = 2;

#[napi]
pub fn gen_salt_sync(round: f64, version: Option<String>) -> Result<String> {
  let round = validate_cost(round)?;
  Ok(format_salt(
    round,
    &version_from_str(version.as_deref())?,
    &gen_salt(),
  ))
}

#[napi(js_name = "genSalt")]
pub fn gen_salt_js(
  round: f64,
  version: Option<String>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<salt_task::SaltTask>> {
  let task = salt_task::SaltTask {
    round: validate_cost(round)?,
    version: version_from_str(version.as_deref())?,
  };
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn hash_sync(
  input: Either<String, &[u8]>,
  cost: Option<f64>,
  salt: Option<Either<String, &[u8]>>,
  version: Option<String>,
  reject_long_passwords: bool,
) -> Result<String> {
  let options = hash_options(cost, salt, version)?;
  HashTask::validate_password(input.as_ref(), reject_long_passwords)?;
  HashTask::hash(input.as_ref(), options.cost, options.salt, options.version)
}

#[napi]
pub fn hash(
  input: Either<String, &[u8]>,
  cost: Option<f64>,
  salt: Option<Either<String, &[u8]>>,
  version: Option<String>,
  reject_long_passwords: bool,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<HashTask>> {
  let options = hash_options(cost, salt, version)?;
  HashTask::validate_password(input.as_ref(), reject_long_passwords)?;
  let task = HashTask {
    password: input.as_ref().to_vec(),
    cost: options.cost,
    salt: options.salt,
    version: options.version,
  };
  Ok(AsyncTask::with_optional_signal(task, signal))
}

#[napi]
pub fn verify_sync(input: Either<String, &[u8]>, hash: Either<String, &[u8]>) -> bool {
  VerifyTask::verify(input.as_ref(), hash.as_ref())
}

#[napi]
pub fn verify(
  password: Either<String, &[u8]>,
  hash: Either<String, &[u8]>,
  signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  let task = VerifyTask {
    password: password.as_ref().to_vec(),
    hash: hash.as_ref().to_vec(),
  };
  Ok(AsyncTask::with_optional_signal(task, signal))
}
