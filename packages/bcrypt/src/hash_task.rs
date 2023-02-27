use napi::{
  bindgen_prelude::{Buffer, Either},
  Env, Error, JsBuffer, JsBufferValue, Ref, Result, Status, Task,
};
use napi_derive::napi;

pub enum AsyncHashInput {
  String(String),
  Buffer(Ref<JsBufferValue>),
}

impl AsyncHashInput {
  #[inline(always)]
  pub fn from_either(input: Either<String, JsBuffer>) -> Result<Self> {
    match input {
      Either::A(s) => Ok(Self::String(s)),
      Either::B(b) => Ok(Self::Buffer(b.into_ref()?)),
    }
  }
}

impl AsRef<[u8]> for AsyncHashInput {
  #[inline(always)]
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
  salt: Option<Buffer>,
}

impl HashTask {
  #[inline(always)]
  pub fn new(buf: AsyncHashInput, cost: u32, salt: Option<Buffer>) -> HashTask {
    HashTask { buf, cost, salt }
  }

  #[inline(always)]
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
    let salt = if let Some(salt) = &self.salt {
      let mut s = [0u8; 16];
      s.copy_from_slice(salt.as_ref());
      s
    } else {
      rand::random()
    };
    match &self.buf {
      AsyncHashInput::String(s) => Self::hash(s.as_bytes(), salt, self.cost),
      AsyncHashInput::Buffer(buf) => Self::hash(buf.as_ref(), salt, self.cost),
    }
  }

  #[inline(always)]
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }

  #[inline(always)]
  fn finally(&mut self, env: Env) -> Result<()> {
    if let AsyncHashInput::Buffer(buf) = &mut self.buf {
      buf.unref(env)?;
    }
    Ok(())
  }
}
