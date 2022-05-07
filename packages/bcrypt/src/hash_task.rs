use bcrypt::Version as BcryptVersion;
use napi::{
  bindgen_prelude::{Buffer, Either},
  Env, Error, JsBuffer, JsBufferValue, Ref, Result, Status, Task,
};
use napi_derive::napi;

use crate::salt_task::gen_salt;

#[napi]
#[allow(non_snake_case)]
pub mod Version {
  #[napi]
  pub const TWO_A: &str = "2a";
  #[napi]
  pub const TWO_B: &str = "2b";
  #[napi]
  pub const TWO_Y: &str = "2y";
  #[napi]
  pub const TWO_X: &str = "2x";
}

#[napi(object)]
#[derive(Default)]
pub struct HashOptions {
  pub salt: Option<Buffer>,
  pub version: Option<String>,
  pub cost: Option<u32>,
}

pub enum AsyncHashInput {
  String(String),
  Buffer(Ref<JsBufferValue>),
}

impl AsyncHashInput {
  #[inline]
  pub fn from_either(input: Either<String, JsBuffer>) -> Result<Self> {
    match input {
      Either::A(s) => Ok(Self::String(s)),
      Either::B(b) => Ok(Self::Buffer(b.into_ref()?)),
    }
  }
}

impl AsRef<[u8]> for AsyncHashInput {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    match self {
      Self::String(s) => s.as_bytes(),
      Self::Buffer(b) => b.as_ref(),
    }
  }
}

pub struct HashTask {
  buf: AsyncHashInput,
  options: HashOptions,
}

impl HashTask {
  #[inline]
  pub fn new(buf: AsyncHashInput, options: HashOptions) -> HashTask {
    HashTask { buf, options }
  }
}

#[napi]
impl Task for HashTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    let salt = if let Some(ref salt) = self.options.salt {
      let mut s = [0; 16];
      s.copy_from_slice(salt.as_ref());
      s
    } else {
      gen_salt().map_err(|err| Error::new(Status::GenericFailure, format!("{}", err)))?
    };
    let cost = self.options.cost.unwrap_or(bcrypt::DEFAULT_COST);
    match &self.buf {
      AsyncHashInput::String(s) => bcrypt::hash_with_salt(s.as_bytes(), cost, salt),
      AsyncHashInput::Buffer(buf) => bcrypt::hash_with_salt(buf.as_ref(), cost, salt),
    }
    .map_err(|err| Error::new(Status::GenericFailure, format!("{}", err)))
    .and_then(|hash_part| {
      if let Some(ref version) = self.options.version {
        Ok(hash_part.format_for_version(version_from_str(version)?))
      } else {
        Ok(hash_part.to_string())
      }
    })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let AsyncHashInput::Buffer(buf) = &mut self.buf {
      buf.unref(env)?;
    }
    Ok(())
  }
}

#[inline]
fn version_from_str(version: &str) -> Result<BcryptVersion> {
  match version {
    "2a" => Ok(BcryptVersion::TwoA),
    "2b" => Ok(BcryptVersion::TwoB),
    "2y" => Ok(BcryptVersion::TwoX),
    "2x" => Ok(BcryptVersion::TwoY),
    _ => Err(Error::new(
      Status::InvalidArg,
      format!("{} is not a valid version", version),
    )),
  }
}
