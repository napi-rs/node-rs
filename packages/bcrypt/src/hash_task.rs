use napi::{
  bindgen_prelude::Either, Env, Error, JsBuffer, JsBufferValue, Ref, Result, Status, Task,
};
use napi_derive::napi;

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
  cost: u32,
  salt: [u8; 16],
}

impl HashTask {
  #[inline]
  pub fn new(buf: AsyncHashInput, cost: u32, salt: [u8; 16]) -> HashTask {
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
    match &self.buf {
      AsyncHashInput::String(s) => Self::hash(s.as_bytes(), self.salt, self.cost),
      AsyncHashInput::Buffer(buf) => Self::hash(buf.as_ref(), self.salt, self.cost),
    }
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
