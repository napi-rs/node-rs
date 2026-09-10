use std::str;

use napi::bindgen_prelude::*;
use napi_derive::napi;

pub struct VerifyTask {
  pub(crate) password: Vec<u8>,
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
