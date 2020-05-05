use napi::{Buffer, Env, Error, JsString, Result, Status, Task, Value};

use crate::lib_bcrypt::hash;

pub struct HashTask {
  buf: Value<Buffer>,
  cost: u32,
}

impl HashTask {
  pub fn new(buf: Value<Buffer>, cost: u32) -> HashTask {
    HashTask { buf, cost }
  }

  #[inline]
  pub fn hash(buf: Value<Buffer>, cost: u32) -> Result<String> {
    hash(buf, cost).map_err(|_| Error::from_status(Status::GenericFailure))
  }
}

impl Task for HashTask {
  type Output = String;
  type JsValue = JsString;

  fn compute(&self) -> Result<Self::Output> {
    Self::hash(self.buf, self.cost)
  }

  fn resolve(&self, env: &mut Env, output: Self::Output) -> Result<Value<Self::JsValue>> {
    env.create_string(&output)
  }
}
