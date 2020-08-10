use std::str;

use crate::lib_bcrypt::verify;
use napi::{Env, Error, JsBoolean, JsBuffer, Result, Status, Task};

pub struct VerifyTask {
  password: &'static [u8],
  hash: &'static [u8],
}

impl VerifyTask {
  pub fn new(password: JsBuffer, hash: JsBuffer) -> VerifyTask {
    Self {
      password: password.data,
      hash: hash.data,
    }
  }

  #[inline]
  pub fn verify(password: &[u8], hash: &[u8]) -> Result<bool> {
    verify(
      &password,
      str::from_utf8(&hash).map_err(|_| Error::from_status(Status::StringExpected))?,
    )
    .map_err(|e| Error::new(Status::GenericFailure, format!("{}", e)))
  }
}

impl Task for VerifyTask {
  type Output = bool;
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    VerifyTask::verify(self.password, self.hash)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
    env.get_boolean(output)
  }
}
