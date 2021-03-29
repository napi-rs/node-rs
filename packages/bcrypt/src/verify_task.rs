use std::str;

use crate::lib_bcrypt::verify;
use napi::{Env, Error, JsBoolean, JsBufferValue, Ref, Result, Status, Task};

pub struct VerifyTask {
  password: Ref<JsBufferValue>,
  hash: Ref<JsBufferValue>,
}

impl VerifyTask {
  pub fn new(password: Ref<JsBufferValue>, hash: Ref<JsBufferValue>) -> VerifyTask {
    Self { password, hash }
  }

  #[inline]
  pub fn verify(password: &[u8], hash: &[u8]) -> Result<bool> {
    Ok(
      verify(
        &password,
        str::from_utf8(&hash).map_err(|_| Error::from_status(Status::StringExpected))?,
      )
      .unwrap_or(false),
    )
  }
}

impl Task for VerifyTask {
  type Output = bool;
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    VerifyTask::verify(&self.password, &self.hash)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    self.password.unref(env)?;
    self.hash.unref(env)?;
    env.get_boolean(output)
  }
}
