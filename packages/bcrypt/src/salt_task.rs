use getrandom::getrandom;
use napi::{Env, Error, Result, Status, Task};
use napi_derive::napi;

use crate::Version;

#[inline]
pub(crate) fn gen_salt() -> bcrypt::BcryptResult<[u8; 16]> {
  let mut s = [0u8; 16];
  getrandom(&mut s)
    .map(|_| s)
    .map_err(bcrypt::BcryptError::from)?;
  Ok(s)
}

#[inline]
pub(crate) fn format_salt(rounds: u32, version: &Version, salt: &[u8; 16]) -> String {
  let mut base64_string = String::new();
  base64::encode_engine_string(
    salt,
    &mut base64_string,
    &base64::engine::fast_portable::FastPortable::from(
      &base64::alphabet::BCRYPT,
      base64::engine::fast_portable::PAD,
    ),
  );
  format!("${}${:0>2}${}", version, rounds, base64_string)
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
    let random = gen_salt().map_err(|err| {
      Error::new(
        Status::GenericFailure,
        format!("Generate salt failed {}", err),
      )
    })?;
    Ok(format_salt(self.round, &self.version, &random))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
