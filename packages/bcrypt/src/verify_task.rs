use std::str;

use napi::bindgen_prelude::*;
use napi_derive::napi;

pub struct VerifyTask {
  password: Either<Uint8Array, String>,
  hash: Either<Uint8Array, String>,
}

impl VerifyTask {
  pub fn new(password: Either<Uint8Array, String>, hash: Either<Uint8Array, String>) -> VerifyTask {
    Self { password, hash }
  }

  #[inline]
  pub fn verify<P, H>(password: P, hash: H) -> Result<bool>
  where
    P: AsRef<[u8]>,
    H: AsRef<[u8]>,
  {
    Ok(
      bcrypt::verify(
        password,
        str::from_utf8(hash.as_ref()).map_err(|_| Error::from_status(Status::StringExpected))?,
      )
      .unwrap_or(false),
    )
  }
}

#[napi]
impl Task for VerifyTask {
  type Output = bool;
  type JsValue = bool;

  fn compute(&mut self) -> Result<Self::Output> {
    VerifyTask::verify(self.password.as_ref(), self.hash.as_ref())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
