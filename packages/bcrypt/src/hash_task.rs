use napi::{Env, Error, JsBuffer, JsString, Result, Status, Task};

use crate::lib_bcrypt::hash;

pub struct HashTask {
  buf: &'static [u8],
  cost: u32,
}

impl HashTask {
  pub fn new(buf: JsBuffer, cost: u32) -> HashTask {
    HashTask {
      buf: buf.data,
      cost,
    }
  }

  #[inline]
  pub fn hash(buf: &[u8], cost: u32) -> Result<String> {
    hash(buf, cost).map_err(|_| Error::from_status(Status::GenericFailure))
  }
}

impl Task for HashTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&mut self) -> Result<Self::Output> {
    Self::hash(self.buf, self.cost)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_string(output.as_str())
  }
}
