#![deny(clippy::all)]

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum Algorithm {
  Argon2d,
  Argon2i,
  Argon2id,
}

impl Algorithm {
  #[inline]
  fn to_argon(self) -> argon2::Algorithm {
    match self {
      Self::Argon2d => argon2::Algorithm::Argon2d,
      Self::Argon2i => argon2::Algorithm::Argon2i,
      Self::Argon2id => argon2::Algorithm::Argon2id,
    }
  }
}

#[napi]
pub enum Version {
  V0x10,
  V0x13,
}

impl Version {
  #[inline]
  fn to_argon(self) -> argon2::Version {
    match self {
      Self::V0x10 => argon2::Version::V0x10,
      Self::V0x13 => argon2::Version::V0x13,
    }
  }
}

#[napi(object)]
#[derive(Default)]
pub struct Options {
  /// Memory size, expressed in kilobytes, between 1 and (2^32)-1.
  /// Value is an integer in decimal (1 to 10 digits).
  pub memory_cost: Option<u32>,
  /// Number of iterations, between 1 and (2^32)-1.
  /// Value is an integer in decimal (1 to 10 digits).
  pub time_cost: Option<u32>,
  /// Degree of parallelism, between 1 and 255.
  /// Value is an integer in decimal (1 to 3 digits).
  pub output_len: Option<u32>,
  pub parallelism: Option<u32>,
  pub algorithm: Option<Algorithm>,
  pub version: Option<Version>,
  pub secret: Option<Buffer>,
}

impl Options {
  #[inline]
  fn to_argon(&self) -> std::result::Result<Argon2, argon2::Error> {
    let algorithm = self.algorithm.map(|a| a.to_argon()).unwrap_or_default();
    let version = self.version.map(|v| v.to_argon()).unwrap_or_default();
    let params = argon2::Params::new(
      self.memory_cost.unwrap_or(argon2::Params::DEFAULT_M_COST),
      self.time_cost.unwrap_or(argon2::Params::DEFAULT_T_COST),
      self.parallelism.unwrap_or(argon2::Params::DEFAULT_P_COST),
      self.output_len.map(|o| o as usize),
    )?;
    if let Some(sec) = &self.secret {
      Argon2::new_with_secret(sec.as_ref(), algorithm, version, params)
    } else {
      Ok(Argon2::new(algorithm, version, params))
    }
  }
}

pub struct HashTask {
  password: Vec<u8>,
  options: Options,
}

#[napi]
impl Task for HashTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = self.options.to_argon();
    hasher
      .map_err(|err| Error::new(Status::InvalidArg, format!("{}", err)))?
      .hash_password(self.password.as_slice(), &salt)
      .map_err(|err| Error::new(Status::GenericFailure, format!("{}", err)))
      .map(|h| h.to_string())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn hash(
  password: Either<String, Buffer>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<HashTask> {
  AsyncTask::with_optional_signal(
    HashTask {
      password: match password {
        Either::A(s) => s.as_bytes().to_vec(),
        Either::B(b) => b.to_vec(),
      },
      options: options.unwrap_or_default(),
    },
    abort_signal,
  )
}

pub struct VerifyTask {
  password: String,
  hashed: String,
  options: Options,
}

#[napi]
impl Task for VerifyTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    let parsed_hash = argon2::PasswordHash::new(self.hashed.as_str())
      .map_err(|err| Error::new(Status::InvalidArg, format!("{}", err)))?;
    let argon2 = self.options.to_argon();
    Ok(
      argon2
        .map_err(|err| Error::new(Status::InvalidArg, format!("{}", err)))?
        .verify_password(self.password.as_bytes(), &parsed_hash)
        .is_ok(),
    )
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn verify(
  hashed: Either<String, Buffer>,
  password: Either<String, Buffer>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  Ok(AsyncTask::with_optional_signal(
    VerifyTask {
      password: match password {
        Either::A(s) => s,
        Either::B(b) => String::from_utf8(b.to_vec())
          .map_err(|err| Error::new(Status::InvalidArg, format!("{}", err)))?,
      },
      hashed: match hashed {
        Either::A(s) => s,
        Either::B(b) => String::from_utf8(b.to_vec())
          .map_err(|err| Error::new(Status::InvalidArg, format!("{}", err)))?,
      },
      options: options.unwrap_or_default(),
    },
    abort_signal,
  ))
}
