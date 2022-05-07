use getrandom::getrandom;
use napi::{bindgen_prelude::Buffer, Env, Error, Result, Status, Task};
use napi_derive::napi;

#[inline]
pub(crate) fn gen_salt() -> bcrypt::BcryptResult<[u8; 16]> {
  let mut s = [0u8; 16];
  getrandom(&mut s)
    .map(|_| s)
    .map_err(bcrypt::BcryptError::from)?;
  Ok(s)
}

#[napi]
pub struct Salt {
  pub(crate) inner: [u8; 16],
  pub(crate) version: bcrypt::Version,
  pub(crate) cost: u32,
}

#[napi]
impl Salt {
  #[napi]
  pub fn version(&self) -> String {
    format!("{}", self.version)
  }

  #[napi]
  pub fn cost(&self) -> u32 {
    self.cost
  }

  #[napi]
  pub fn to_string(&self) -> String {
    base64::encode_config(self.inner, base64::BCRYPT)
  }
}

pub struct SaltTask {}

#[napi]
impl Task for SaltTask {
  type Output = [u8; 16];
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let random = gen_salt().map_err(|err| {
      Error::new(
        Status::GenericFailure,
        format!("Generate salt failed {}", err),
      )
    })?;
    Ok(random)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.to_vec().into())
  }
}
