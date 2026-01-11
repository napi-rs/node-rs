use napi::{
  Env, Error, Result, Status, Task,
  bindgen_prelude::{Either, Uint8Array},
};
use napi_derive::napi;

pub struct HashTask {
  buf: Either<Uint8Array, String>,
  cost: u32,
  salt: [u8; 16],
}

impl HashTask {
  #[inline]
  pub fn new(buf: Either<Uint8Array, String>, cost: u32, salt: [u8; 16]) -> HashTask {
    HashTask { buf, cost, salt }
  }

  #[inline]
  pub fn hash(buf: &[u8], salt: [u8; 16], cost: u32) -> Result<String> {
    bcrypt::hash_with_salt(buf, cost, salt)
      .map(|hash_part| hash_part.to_string())
      .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
  }
}

#[napi]
impl Task for HashTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    Self::hash(self.buf.as_ref(), self.salt, self.cost)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
