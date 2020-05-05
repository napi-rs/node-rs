use std::str;

use crate::lib_bcrypt::verify;
use napi::{Boolean, Buffer, Env, Error, Result, Status, Task, Value};

pub struct VerifyTask {
  password: Value<Buffer>,
  hash: Value<Buffer>,
}

impl VerifyTask {
  pub fn new(password: Value<Buffer>, hash: Value<Buffer>) -> VerifyTask {
    Self { password, hash }
  }

  #[inline]
  pub fn verify(password: Value<Buffer>, hash: Value<Buffer>) -> Result<bool> {
    verify(
      &password,
      str::from_utf8(&hash).map_err(|_| Error::from_status(Status::StringExpected))?,
    )
    .map_err(|_| Error::from_status(Status::GenericFailure))
  }
}

impl Task for VerifyTask {
  type Output = bool;
  type JsValue = Boolean;

  fn compute(&self) -> Result<Self::Output> {
    VerifyTask::verify(self.password, self.hash)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Value<Self::JsValue>> {
    env.get_boolean(output)
  }
}
