#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

use argon2::{
  password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};

#[napi]
pub enum Algorithm {
  /// Optimizes against GPU cracking attacks but vulnerable to side-channels.
  /// Accesses the memory array in a password dependent order, reducing the possibility of time–memory tradeoff (TMTO) attacks.
  Argon2d,

  /// Optimized to resist side-channel attacks.
  /// Accesses the memory array in a password independent order, increasing the possibility of time-memory tradeoff (TMTO) attacks.
  Argon2i,

  /// Default value, this is the default algorithm for normative recommendations.
  /// Hybrid that mixes Argon2i and Argon2d passes.
  /// Uses the Argon2i approach for the first half pass over memory and Argon2d approach for subsequent passes. This effectively places it in the “middle” between the other two: it doesn’t provide as good TMTO/GPU cracking resistance as Argon2d, nor as good of side-channel resistance as Argon2i, but overall provides the most well-rounded approach to both classes of attacks.
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
  /// Version 16 (0x10 in hex)
  V0x10,

  /// Default value
  /// Version 19 (0x13 in hex)
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
  /// The amount of memory to be used by the hash function, in kilobytes. Each thread will have a memory pool of this size. Note that large values for highly concurrent usage will cause starvation and thrashing if your system memory gets full.
  ///
  /// Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
  ///
  /// The default value is 4096, meaning a pool of 4 MiB per thread.
  pub memory_cost: Option<u32>,

  /// The time cost is the amount of passes (iterations) used by the hash function. It increases hash strength at the cost of time required to compute.
  ///
  /// Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
  ///
  /// The default value is 3.
  pub time_cost: Option<u32>,

  /// The hash length is the length of the hash function output in bytes. Note that the resulting hash is encoded with Base 64, so the digest will be ~1/3 longer.
  ///
  /// The default value is 32, which produces raw hashes of 32 bytes or digests of 43 characters.
  pub output_len: Option<u32>,

  /// The amount of threads to compute the hash on. Each thread has a memory pool with memoryCost size. Note that changing it also changes the resulting hash.
  ///
  /// Value is an integer in decimal (1 to 3 digits), between 1 and 255.
  ///
  /// The default value is 1, meaning a single thread is used.
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
