use base64::engine::Engine;
use napi::{Env, Result, Task};
use napi_derive::napi;

use bcrypt::Version;

#[inline]
pub(crate) fn gen_salt() -> [u8; 16] {
  rand::random()
}

#[inline]
pub(crate) fn format_salt(rounds: u32, version: &Version, salt: &[u8; 16]) -> String {
  let base64_string = salt_engine().encode(salt);
  format!("${version}${rounds:0>2}${base64_string}")
}

pub(crate) fn salt_engine() -> base64::engine::general_purpose::GeneralPurpose {
  base64::engine::general_purpose::GeneralPurpose::new(
    &base64::alphabet::BCRYPT,
    base64::engine::general_purpose::NO_PAD,
  )
}

pub struct SaltTask {
  pub(crate) round: u32,
  pub(crate) version: Version,
}

#[napi]
impl Task for SaltTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    let random = gen_salt();
    Ok(format_salt(self.round, &self.version, &random))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
