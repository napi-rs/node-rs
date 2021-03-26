use napi::{Env, JsString, Result, Task};

use crate::{format_salt, gen_salt, Version};

pub struct SaltTask {
  pub(crate) round: u32,
  pub(crate) version: Version,
}

impl Task for SaltTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    let random = gen_salt();
    Ok(format_salt(self.round, self.version, &random))
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(output.as_str())
  }
}
