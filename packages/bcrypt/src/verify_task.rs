use std::str;

use base64::engine::Engine;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use zeroize::Zeroizing;

use crate::salt_task::salt_engine;

const HASH_STRING_LEN: usize = 60;

/// A stored hash read with the verifier's lenient parser: `bcrypt-rust`'s
/// `HashParts::from_str` is deliberately strict, but verification must keep
/// accepting every hash the previously shipped backend accepted (e.g. the
/// noncanonical `+4` cost spelling that `str::parse::<u32>` allows).
/// Cost is not range-checked here — a parseable but out-of-range cost makes
/// verification return `Ok(false)`-equivalent, matching the old backend,
/// which rejected it at hash time rather than at parse time.
struct StoredHash {
  cost: u32,
  salt: [u8; 16],
  hash: [u8; 23],
}

fn split_hash_lenient(hash: &[u8]) -> Option<StoredHash> {
  // Same contract as the old backend's parser: exactly 60 ASCII bytes with
  // `$` separators, a `$2a$`-family version marker, and both payload fields
  // valid bcrypt base64.
  if hash.len() != HASH_STRING_LEN || !hash.is_ascii() {
    return None;
  }
  if hash[0] != b'$' || hash[3] != b'$' || hash[6] != b'$' {
    return None;
  }
  if hash[1] != b'2' || !matches!(hash[2], b'a' | b'b' | b'x' | b'y') {
    return None;
  }
  // `u32::parse` accepts an optional leading `+`; that leniency is load-bearing
  // for hashes written by other implementations.
  let cost = str::from_utf8(&hash[4..6]).ok()?.parse::<u32>().ok()?;
  let salt: [u8; 16] = salt_engine().decode(&hash[7..29]).ok()?.try_into().ok()?;
  let hash_bytes: [u8; 23] = salt_engine().decode(&hash[29..60]).ok()?.try_into().ok()?;
  Some(StoredHash {
    cost,
    salt,
    hash: hash_bytes,
  })
}

/// Prefix and cost of a stored hash, read with the verifier's parser.
#[napi(object)]
pub struct ParsedHashOptions {
  pub version: String,
  pub cost: u32,
}

/// Uses the same parser as verification, so every hash `verify` can accept is parseable,
/// including noncanonical costs and imported `2x` labels. Hashes `verify` always rejects
/// are errors here rather than unusable parameters.
pub(crate) fn parse_stored_hash(hash: &[u8]) -> Result<ParsedHashOptions> {
  let invalid = |message| Error::new(Status::InvalidArg, message);
  let encoded =
    str::from_utf8(hash).map_err(|_| invalid("hash must be a bcrypt hash accepted by verify"))?;
  let parts = split_hash_lenient(hash)
    .ok_or_else(|| invalid("hash must be a bcrypt hash accepted by verify"))?;
  if !(4..=31).contains(&parts.cost) {
    return Err(invalid("hash cost must be between 4 and 31"));
  }
  // The parser guarantees 60 ASCII bytes with `$2` at the start.
  Ok(ParsedHashOptions {
    version: encoded[1..3].to_string(),
    cost: parts.cost,
  })
}

pub struct VerifyTask {
  pub(crate) password: Zeroizing<Vec<u8>>,
  pub(crate) hash: Vec<u8>,
}

impl VerifyTask {
  pub fn verify(password: &[u8], hash: &[u8]) -> bool {
    let Some(parts) = split_hash_lenient(hash) else {
      return false;
    };
    // Canonicalize and hand off to the backend's own verifier, so the cost
    // range gate and the constant-time compare stay in the library. A
    // parseable-but-out-of-range cost surfaces as `Err` → `false`, matching
    // the old backend's hash-time rejection.
    let canonical = format!(
      "$2b${:02}${}{}",
      parts.cost,
      salt_engine().encode(parts.salt),
      salt_engine().encode(parts.hash),
    );
    bcrypt::verify(password, &canonical).unwrap_or(false)
  }
}

#[napi]
impl Task for VerifyTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(Self::verify(&self.password, &self.hash))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
