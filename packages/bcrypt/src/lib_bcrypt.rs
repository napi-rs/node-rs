//! Easily hash and verify passwords using bcrypt
use rand::{rngs::OsRng, RngCore};
use std::fmt;
use std::str::FromStr;

pub use crate::errors::{BcryptError, BcryptResult};

#[derive(Debug, PartialEq)]
/// A bcrypt hash result before concatenating
pub struct HashParts {
  cost: u32,
  salt: String,
  hash: String,
}

/// BCrypt hash version
/// https://en.wikipedia.org/wiki/Bcrypt#Versioning_history
#[derive(Debug, Clone, Copy)]
pub enum Version {
  A,
  X,
  Y,
  B,
}

impl FromStr for Version {
  type Err = BcryptError;

  fn from_str(s: &str) -> Result<Version, Self::Err> {
    if s == "2a" {
      return Ok(Version::A);
    }
    if s == "2x" {
      return Ok(Version::X);
    }
    if s == "2y" {
      return Ok(Version::Y);
    }
    if s == "2b" {
      return Ok(Version::B);
    }
    Err(BcryptError::InvalidVersion(s.to_owned()))
  }
}

impl HashParts {
  /// Get the bcrypt hash cost
  #[allow(dead_code)]
  pub fn get_cost(&self) -> u32 {
    self.cost
  }

  /// Get the bcrypt hash salt
  #[allow(dead_code)]
  pub fn get_salt(&self) -> String {
    self.salt.clone()
  }

  /// Creates the bcrypt hash string from all its part, allowing to customize the version.
  pub fn format_for_version(&self, version: Version) -> String {
    // Cost need to have a length of 2 so padding with a 0 if cost < 10
    format!("${}${:02}${}{}", version, self.cost, self.salt, self.hash)
  }
}

impl FromStr for HashParts {
  type Err = BcryptError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    split_hash(s)
  }
}

impl ToString for HashParts {
  fn to_string(&self) -> String {
    self.format_for_version(Version::Y)
  }
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      Version::A => "2a",
      Version::B => "2b",
      Version::X => "2x",
      Version::Y => "2y",
    };
    write!(f, "{}", str)
  }
}

#[inline]
pub fn gen_salt() -> [u8; 16] {
  let mut s = [0u8; 16];
  OsRng.fill_bytes(&mut s);
  s
}

#[inline]
pub fn format_salt(rounds: u32, version: Version, salt: &[u8; 16]) -> String {
  format!("${}${:0>2}${}", version, rounds, crate::b64::encode(salt))
}

/// Takes a full hash and split it into 3 parts:
/// cost, salt and hash
fn split_hash(hash: &str) -> BcryptResult<HashParts> {
  let mut parts = HashParts {
    cost: 0,
    salt: "".to_string(),
    hash: "".to_string(),
  };

  // Should be [prefix, cost, hash]
  let raw_parts: Vec<_> = hash.split('$').filter(|s| !s.is_empty()).collect();

  if raw_parts.len() != 3 {
    return Err(BcryptError::InvalidHash(hash.to_string()));
  }

  if raw_parts[0] != "2y" && raw_parts[0] != "2b" && raw_parts[0] != "2a" {
    return Err(BcryptError::InvalidPrefix(raw_parts[0].to_string()));
  }

  if let Ok(c) = raw_parts[1].parse::<u32>() {
    parts.cost = c;
  } else {
    return Err(BcryptError::InvalidCost(raw_parts[1].to_string()));
  }

  if raw_parts[2].len() == 53 {
    parts.salt = raw_parts[2][..22].chars().collect();
    parts.hash = raw_parts[2][22..].chars().collect();
  } else {
    return Err(BcryptError::InvalidHash(hash.to_string()));
  }

  Ok(parts)
}
