use std::str;

use napi::{Env, Error, JsBoolean, Result, Status, Task};
use napi_derive::napi;

use crate::hash_task::AsyncHashInput;

pub struct VerifyTask {
  password: AsyncHashInput,
  hash: AsyncHashInput,
}

impl VerifyTask {
  pub fn new(password: AsyncHashInput, hash: AsyncHashInput) -> VerifyTask {
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
  type JsValue = JsBoolean;

  fn compute(&mut self) -> Result<Self::Output> {
    VerifyTask::verify(self.password.as_ref(), self.hash.as_ref())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.get_boolean(output)
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let crate::hash_task::AsyncHashInput::Buffer(buf) = &mut self.password {
      buf.unref(env)?;
    }
    if let crate::hash_task::AsyncHashInput::Buffer(buf) = &mut self.hash {
      buf.unref(env)?;
    }
    Ok(())
  }
}
