use bcrypt::Version;
use napi::{Env, Error, Result, Status, Task};
use napi_derive::napi;
use zeroize::Zeroizing;

/// bcrypt never reads past this many password bytes.
const MAX_PASSWORD_BYTES: usize = 72;

/// Owned copy for async work. Only the bytes bcrypt reads are kept, so the hash is
/// unchanged, and the copy is wiped when the task is dropped.
pub(crate) fn owned_password(password: &[u8]) -> Zeroizing<Vec<u8>> {
  Zeroizing::new(password[..password.len().min(MAX_PASSWORD_BYTES)].to_vec())
}

pub struct HashTask {
  pub(crate) password: Zeroizing<Vec<u8>>,
  pub(crate) cost: u32,
  pub(crate) salt: [u8; 16],
  pub(crate) version: Version,
}

impl HashTask {
  pub fn validate_password(password: &[u8], reject_long_passwords: bool) -> Result<()> {
    if reject_long_passwords && password.len() > MAX_PASSWORD_BYTES {
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
    Self::hash(&self.password, self.cost, self.salt, self.version)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
