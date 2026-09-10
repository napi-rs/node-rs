use bcrypt::Version;
use napi::{Env, Error, Result, Status, Task};
use napi_derive::napi;

pub struct HashTask {
  pub(crate) password: Vec<u8>,
  pub(crate) cost: u32,
  pub(crate) salt: [u8; 16],
  pub(crate) version: Version,
}

impl HashTask {
  pub fn validate_password(password: &[u8], reject_long_passwords: bool) -> Result<()> {
    if reject_long_passwords && password.len() > 72 {
      return Err(Error::new(
        Status::InvalidArg,
        "password must not exceed 72 bytes",
      ));
    }
    Ok(())
  }

  pub fn hash(password: &[u8], cost: u32, salt: [u8; 16], version: Version) -> Result<String> {
    bcrypt::hash_with_salt(password, cost, salt)
      .map(|parts| parts.format_for_version(version))
      .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
  }
}

#[napi]
impl Task for HashTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    Self::hash(&self.password, self.cost, self.salt, self.version.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
