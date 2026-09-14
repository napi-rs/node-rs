use base64::engine::Engine;
use bcrypt::Version;
use napi::bindgen_prelude::*;

use crate::DEFAULT_COST;
use crate::salt_task::{gen_salt, salt_engine};

pub(crate) struct HashOptions {
  pub cost: u32,
  pub salt: [u8; 16],
  pub version: Version,
}

pub(crate) fn validate_cost(cost: f64) -> Result<u32> {
  if !cost.is_finite() || cost.fract() != 0.0 || !(4.0..=31.0).contains(&cost) {
    return Err(Error::new(
      Status::InvalidArg,
      "cost must be an integer between 4 and 31",
    ));
  }
  Ok(cost as u32)
}

pub(crate) fn version_from_str(version: Option<&str>) -> Result<Version> {
  match version {
    Some("2a") => Ok(Version::TwoA),
    Some("2b") | None => Ok(Version::TwoB),
    Some("2y") => Ok(Version::TwoY),
    _ => Err(Error::new(
      Status::InvalidArg,
      "version must be 2a, 2b, or 2y",
    )),
  }
}

// Creation is strict. Never use this parser to gate verification of stored hashes.
pub(crate) fn hash_options(
  cost: Option<f64>,
  salt: Option<Either<String, &[u8]>>,
  version: Option<String>,
) -> Result<HashOptions> {
  if let Some(Either::A(encoded)) = salt.as_ref() {
    if cost.is_some() || version.is_some() {
      return Err(Error::new(
        Status::InvalidArg,
        "an encoded salt already supplies cost and version",
      ));
    }
    let invalid = || {
      Error::new(
        Status::InvalidArg,
        "salt must be a canonical 29-character bcrypt salt",
      )
    };
    if encoded.len() != 29 || !encoded.is_ascii() {
      return Err(invalid());
    }
    let bytes = encoded.as_bytes();
    if bytes[0] != b'$'
      || bytes[3] != b'$'
      || bytes[6] != b'$'
      || !bytes[4].is_ascii_digit()
      || !bytes[5].is_ascii_digit()
    {
      return Err(invalid());
    }
    let version = version_from_str(Some(&encoded[1..3]))?;
    let cost = validate_cost(((bytes[4] - b'0') * 10 + bytes[5] - b'0') as f64)?;
    let decoded = salt_engine().decode(&encoded[7..]).map_err(|_| invalid())?;
    let salt: [u8; 16] = decoded.try_into().map_err(|_| invalid())?;
    if salt_engine().encode(salt) != encoded[7..] {
      return Err(invalid());
    }
    return Ok(HashOptions {
      cost,
      salt,
      version,
    });
  }
  let cost = validate_cost(cost.unwrap_or(DEFAULT_COST as f64))?;
  let version = version_from_str(version.as_deref())?;
  let salt = match salt {
    Some(Either::B(bytes)) => bytes
      .try_into()
      .map_err(|_| Error::new(Status::InvalidArg, "raw salt must contain exactly 16 bytes"))?,
    None => gen_salt(),
    Some(Either::A(_)) => unreachable!(),
  };
  Ok(HashOptions {
    cost,
    salt,
    version,
  })
}
