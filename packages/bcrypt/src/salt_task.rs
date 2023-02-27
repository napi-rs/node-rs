use base64::Engine;
use napi::{Env, Result, Task};
use napi_derive::napi;

use crate::Version;

const BASE64_ENCODE_BCRYPT: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
  &base64::alphabet::BCRYPT,
  base64::engine::GeneralPurposeConfig::new().with_encode_padding(true),
);

#[inline(always)]
pub(crate) fn format_salt(rounds: u32, version: &Version, salt: &[u8; 16]) -> String {
  let mut base64_string = String::new();
  BASE64_ENCODE_BCRYPT.encode_string(salt, &mut base64_string);
  format!("${version}${rounds:0>2}${base64_string}")
}

pub struct SaltTask {
  pub(crate) round: u32,
  pub(crate) version: Version,
}

#[napi]
impl Task for SaltTask {
  type Output = String;
  type JsValue = String;

  #[inline(always)]
  fn compute(&mut self) -> Result<Self::Output> {
    let random = rand::random();
    Ok(format_salt(self.round, &self.version, &random))
  }

  #[inline(always)]
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
