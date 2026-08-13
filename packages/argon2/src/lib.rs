#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use argon2_rust::{
  Algorithm as Argon2Algorithm, Argon2, Error as Argon2Error, Params, Version as Argon2Version,
  params::{Memory, TagLen},
};
use base64::Engine;
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
      builder = builder.lanes(parallelism);
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
      String::from_utf8(b.to_vec()).map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))
    }
  }
}

fn generate_salt() -> [u8; argon2_rust::RANDOM_SALT_LEN] {
  rand::random()
}

fn encode_phc(argon2: &Argon2, salt: &[u8], tag: &[u8]) -> String {
  format!(
    "${}$v={}$m={},t={},p={}${}${}",
    argon2.algorithm().as_str(),
    argon2.version().as_u32(),
    argon2.params().memory_kib(),
    argon2.params().passes(),
    argon2.params().lanes(),
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(salt),
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(tag),
  )
}

fn hash_encoded(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<String> {
  if secret.is_empty() {
    return argon2.hash_encoded(password, salt).map_err(map_error);
  }
  let mut tag = vec![0u8; argon2.params().tag_len_bytes()];
  argon2
    .hash_into_with_ad(password, salt, secret, &[], &mut tag)
    .map_err(map_error)?;
  Ok(encode_phc(argon2, salt, &tag))
}

fn hash_raw_bytes(argon2: &Argon2, password: &[u8], salt: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
  let mut tag = vec![0u8; argon2.params().tag_len_bytes()];
  argon2
    .hash_into_with_ad(password, salt, secret, &[], &mut tag)
    .map_err(map_error)?;
  Ok(tag)
}

fn decode_fail<T>() -> Result<T> {
  Err(Error::new(
    Status::InvalidArg,
    format!("Invalid hashed password: {}", Argon2Error::DecodingFail),
  ))
}

fn parse_phc_u32(value: &str) -> Result<u32> {
  value.parse().or_else(|_| decode_fail())
}

fn decode_b64(input: &str) -> Result<Vec<u8>> {
  base64::engine::general_purpose::STANDARD_NO_PAD
    .decode(input)
    .map_err(|_| {
      Error::new(
        Status::InvalidArg,
        format!("Invalid hashed password: {}", Argon2Error::DecodingFail),
      )
    })
}

struct DecodedPhc {
  algorithm: Argon2Algorithm,
  version: Argon2Version,
  params: Params,
  salt: Vec<u8>,
  hash: Vec<u8>,
  ad: Vec<u8>,
}

/// PHC strings from the C reference / argon2-rust use `m,t,p`.
/// `@phc/format` (node-argon2) serializes object insertion order `m,p,t`.
/// Accept either, plus a missing `$v=` (version 16).
fn decode_phc(encoded: &str) -> Result<DecodedPhc> {
  let mut parts = encoded.split('$');
  if parts.next() != Some("") {
    return decode_fail();
  }

  let algorithm = match parts.next() {
    Some("argon2id") => Argon2Algorithm::Argon2id,
    Some("argon2i") => Argon2Algorithm::Argon2i,
    Some("argon2d") => Argon2Algorithm::Argon2d,
    _ => return decode_fail(),
  };

  let mut next = match parts.next() {
    Some(part) => part,
    None => return decode_fail(),
  };

  let version = if let Some(raw) = next.strip_prefix("v=") {
    next = match parts.next() {
      Some(part) => part,
      None => return decode_fail(),
    };
    match raw.parse::<u32>() {
      Ok(0x10) => Argon2Version::V0x10,
      Ok(0x13) => Argon2Version::V0x13,
      _ => return decode_fail(),
    }
  } else {
    Argon2Version::V0x10
  };

  let mut memory = None;
  let mut passes = None;
  let mut lanes = None;
  let mut ad = Vec::new();
  for field in next.split(',') {
    let Some((key, value)) = field.split_once('=') else {
      return decode_fail();
    };
    match key {
      "m" => memory = Some(parse_phc_u32(value)?),
      "t" => passes = Some(parse_phc_u32(value)?),
      "p" => lanes = Some(parse_phc_u32(value)?),
      // node-argon2 / @phc/format stores associated data here. It is not
      // produced by this binding, but verify must honour it.
      "data" => ad = decode_b64(value)?,
      _ => {}
    }
  }

  let (Some(memory), Some(passes), Some(lanes)) = (memory, passes, lanes) else {
    return decode_fail();
  };

  let salt = decode_b64(match parts.next() {
    Some(part) => part,
    None => return decode_fail(),
  })?;
  let hash = decode_b64(match parts.next() {
    Some(part) => part,
    None => return decode_fail(),
  })?;
  if parts.next().is_some() {
    return decode_fail();
  }

  let params = Params::builder()
    .memory(Memory::kib(memory as u64))
    .passes(passes)
    .lanes(lanes)
    .tag_len(TagLen::bytes(hash.len() as u64))
    .build()
    .map_err(map_error)?;

  Ok(DecodedPhc {
    algorithm,
    version,
    params,
    salt,
    hash,
    ad,
  })
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
  if a.len() != b.len() {
    return false;
  }
  let mut diff = 0u8;
  for (left, right) in a.iter().zip(b) {
    diff |= left ^ right;
  }
  diff == 0
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
    let decoded = decode_phc(&self.hashed)?;
    let argon2 = Argon2::new(decoded.algorithm, decoded.version, decoded.params);
    let secret = self.options.secret();
    let mut computed = vec![0u8; decoded.hash.len()];
    argon2
      .hash_into_with_ad(
        self.password.as_bytes(),
        &decoded.salt,
        secret,
        &decoded.ad,
        &mut computed,
      )
      .map_err(map_error)?;
    Ok(constant_time_eq(&computed, &decoded.hash))
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
