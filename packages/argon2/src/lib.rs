#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use argon2_rust::{
  Algorithm as Argon2Algorithm, Argon2, Error as Argon2Error, Params, Version as Argon2Version,
  params::{Memory, TagLen},
};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone, Copy)]
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
  fn to_argon(self) -> Argon2Algorithm {
    match self {
      Self::Argon2d => Argon2Algorithm::Argon2d,
      Self::Argon2i => Argon2Algorithm::Argon2i,
      Self::Argon2id => Argon2Algorithm::Argon2id,
    }
  }
}

#[napi]
#[derive(Clone, Copy)]
pub enum Version {
  /// Version 16 (0x10 in hex)
  V0x10,

  /// Default value
  /// Version 19 (0x13 in hex)
  V0x13,
}

impl Version {
  #[inline]
  fn to_argon(self) -> Argon2Version {
    match self {
      Self::V0x10 => Argon2Version::V0x10,
      Self::V0x13 => Argon2Version::V0x13,
    }
  }
}

#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct Options {
  /// The amount of memory to be used by the hash function, in kilobytes. Each thread will have a memory pool of this size. Note that large values for highly concurrent usage will cause starvation and thrashing if your system memory gets full.
  ///
  /// Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
  ///
  /// The default value is 19456, meaning a pool of 19 MiB per thread.
  pub memory_cost: Option<u32>,

  /// The time cost is the amount of passes (iterations) used by the hash function. It increases hash strength at the cost of time required to compute.
  ///
  /// Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
  ///
  /// The default value is 2.
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
  pub secret: Option<Uint8Array>,
  pub salt: Option<Uint8Array>,
}

impl Options {
  fn algorithm(&self) -> Argon2Algorithm {
    self
      .algorithm
      .map(|algorithm| algorithm.to_argon())
      .unwrap_or_default()
  }

  fn version(&self) -> Argon2Version {
    self
      .version
      .map(|version| version.to_argon())
      .unwrap_or_default()
  }

  fn secret(&self) -> &[u8] {
    self
      .secret
      .as_ref()
      .map(|secret| secret.as_ref())
      .unwrap_or(&[])
  }

  fn salt(&self) -> Option<&[u8]> {
    self.salt.as_ref().map(|salt| salt.as_ref())
  }

  fn params(&self) -> Result<Params> {
    let mut builder = Params::builder();
    if let Some(memory_cost) = self.memory_cost {
      builder = builder.memory(Memory::kib(memory_cost as u64));
    }
    if let Some(time_cost) = self.time_cost {
      builder = builder.passes(time_cost);
    }
    if let Some(parallelism) = self.parallelism {
      builder = builder
        .lanes(parallelism)
        .threads(thread_budget(parallelism));
    }
    if let Some(output_len) = self.output_len {
      builder = builder.tag_len(TagLen::bytes(output_len as u64));
    }
    builder.build().map_err(map_error)
  }

  fn hasher(&self) -> Result<Argon2> {
    Ok(Argon2::new(
      self.algorithm(),
      self.version(),
      self.params()?,
    ))
  }
}

fn thread_budget(lanes: u32) -> u32 {
  let available = std::thread::available_parallelism()
    .map(|n| n.get() as u32)
    .unwrap_or(1)
    .max(1);
  lanes.min(available)
}

fn map_error(err: Argon2Error) -> Error {
  let status = match err {
    Argon2Error::DecodingFail | Argon2Error::EncodingFail => Status::InvalidArg,
    Argon2Error::MemoryAllocationError
    | Argon2Error::ThreadFail
    | Argon2Error::OsRandom
    | Argon2Error::VerifyMismatch => Status::GenericFailure,
    _ => Status::InvalidArg,
  };
  Error::new(status, err.to_string())
}

fn password_bytes(password: Either<String, &[u8]>) -> Vec<u8> {
  match password {
    Either::A(s) => s.into_bytes(),
    Either::B(b) => b.to_vec(),
  }
}

fn utf8_input(value: Either<String, &[u8]>) -> Result<String> {
  match value {
    Either::A(s) => Ok(s),
    Either::B(b) => {
      simdutf8::basic::from_utf8(b)
        .map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))?;
      // SAFETY: `from_utf8` just accepted these bytes as UTF-8.
      Ok(unsafe { String::from_utf8_unchecked(b.to_vec()) })
    }
  }
}

fn generate_salt() -> [u8; argon2_rust::RANDOM_SALT_LEN] {
  rand::random()
}

fn hash_encoded(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<String> {
  argon2
    .hash_encoded_with_ad(password, salt, secret, &[])
    .map_err(map_error)
}

fn hash_raw_bytes(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
  argon2
    .hash_with_ad(password, salt, secret, &[])
    .map_err(map_error)
}

fn decode_hashed(encoded: &str) -> Result<argon2_rust::Decoded> {
  let mut decoded = argon2_rust::decode_phc(encoded).map_err(map_error)?;
  decoded.params = decoded
    .params
    .to_builder()
    .threads(thread_budget(decoded.params.lanes()))
    .build()
    .map_err(map_error)?;
  Ok(decoded)
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
    let hasher = self.options.hasher()?;
    let secret = self.options.secret();
    match self.options.salt() {
      Some(salt) => hash_encoded(&hasher, &self.password, salt, secret),
      None => {
        let salt = generate_salt();
        hash_encoded(&hasher, &self.password, &salt, secret)
      }
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn hash(
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<HashTask> {
  AsyncTask::with_optional_signal(
    HashTask {
      password: password_bytes(password),
      options: options.unwrap_or_default(),
    },
    abort_signal,
  )
}

#[napi]
pub fn hash_sync(
  env: Env,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<String> {
  let mut hash_task = HashTask {
    password: password_bytes(password),
    options: options.unwrap_or_default(),
  };
  let output = hash_task.compute()?;
  hash_task.resolve(env, output)
}

pub struct RawHashTask {
  password: Vec<u8>,
  options: Options,
}

#[napi]
impl Task for RawHashTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let hasher = self.options.hasher()?;
    let secret = self.options.secret();
    match self.options.salt() {
      Some(salt) => hash_raw_bytes(&hasher, &self.password, salt, secret),
      None => {
        let salt = generate_salt();
        hash_raw_bytes(&hasher, &self.password, &salt, secret)
      }
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi]
pub fn hash_raw(
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> AsyncTask<RawHashTask> {
  AsyncTask::with_optional_signal(
    RawHashTask {
      password: password_bytes(password),
      options: options.unwrap_or_default(),
    },
    abort_signal,
  )
}

#[napi]
pub fn hash_raw_sync(
  env: Env,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<Buffer> {
  let mut hash_task = RawHashTask {
    password: password_bytes(password),
    options: options.unwrap_or_default(),
  };
  let output = hash_task.compute()?;
  hash_task.resolve(env, output)
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
    let decoded = decode_hashed(&self.hashed)?;
    let argon2 = Argon2::new(decoded.algorithm, decoded.version, decoded.params);
    match argon2.verify_with_ad(
      self.password.as_bytes(),
      &decoded.salt,
      self.options.secret(),
      &decoded.ad,
      &decoded.hash,
    ) {
      Ok(()) => Ok(true),
      Err(Argon2Error::VerifyMismatch) => Ok(false),
      Err(err) => Err(map_error(err)),
    }
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn verify(
  hashed: Either<String, &[u8]>,
  password: Either<String, &[u8]>,
  options: Option<Options>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  Ok(AsyncTask::with_optional_signal(
    VerifyTask {
      password: utf8_input(password)?,
      hashed: utf8_input(hashed)?,
      options: options.unwrap_or_default(),
    },
    abort_signal,
  ))
}

#[napi]
pub fn verify_sync(
  env: Env,
  hashed: Either<String, &[u8]>,
  password: Either<String, &[u8]>,
  options: Option<Options>,
) -> Result<bool> {
  let mut verify_task = VerifyTask {
    password: utf8_input(password)?,
    hashed: utf8_input(hashed)?,
    options: options.unwrap_or_default(),
  };
  let output = verify_task.compute()?;
  verify_task.resolve(env, output)
}
