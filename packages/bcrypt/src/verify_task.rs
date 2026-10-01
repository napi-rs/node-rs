use std::str;

use bcrypt::HashParts;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use zeroize::Zeroizing;

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
  let parts: HashParts = encoded
    .parse()
    .map_err(|_| invalid("hash must be a bcrypt hash accepted by verify"))?;
  let cost = parts.get_cost();
  if !(4..=31).contains(&cost) {
    return Err(invalid("hash cost must be between 4 and 31"));
  }
  // The verifier's parser guarantees 60 ASCII bytes with `$` at offsets 0 and 3.
  Ok(ParsedHashOptions {
    version: encoded[1..3].to_string(),
    cost,
  })
}

pub struct VerifyTask {
  pub(crate) password: Zeroizing<Vec<u8>>,
  pub(crate) hash: Vec<u8>,
}

impl VerifyTask {
  pub fn verify(password: &[u8], hash: &[u8]) -> bool {
    let Ok(encoded) = str::from_utf8(hash) else {
      return false;
    };
    // Retain the backend's existing parser, prefix handling and 72-byte semantics.
    bcrypt::verify(password, encoded).unwrap_or(false)
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
