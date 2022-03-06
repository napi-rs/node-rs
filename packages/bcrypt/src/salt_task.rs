use napi::{Env, Error, Result, Status, Task};
use napi_derive::napi;

use crate::{format_salt, gen_salt, Version};

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
    Ok(format_salt(self.round, self.version, &random))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
