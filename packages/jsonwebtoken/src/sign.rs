use napi::{bindgen_prelude::*, JsBuffer, JsBufferValue, Ref};
use napi_derive::napi;

use crate::{claims::Claims, header::Header};

pub enum AsyncKeyInput {
  String(String),
  Buffer(Ref<JsBufferValue>),
}

impl AsyncKeyInput {
  #[inline]
  pub fn from_either(input: Either<String, JsBuffer>) -> Result<Self> {
    match input {
      Either::A(s) => Ok(Self::String(s)),
      Either::B(b) => Ok(Self::Buffer(b.into_ref()?)),
    }
  }
}

impl AsRef<[u8]> for AsyncKeyInput {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    match self {
      Self::String(s) => s.as_bytes(),
      Self::Buffer(b) => b.as_ref(),
    }
  }
}

#[inline]
fn into_encoding_key(
  value: &[u8],
  alg: &jsonwebtoken::Algorithm,
) -> Result<jsonwebtoken::EncodingKey> {
  let encoding_key = match alg {
    // HMAC family
    jsonwebtoken::Algorithm::HS256
    | jsonwebtoken::Algorithm::HS384
    | jsonwebtoken::Algorithm::HS512 => Ok(jsonwebtoken::EncodingKey::from_secret(value)),
    // RSA family
    jsonwebtoken::Algorithm::RS256
    | jsonwebtoken::Algorithm::RS384
    | jsonwebtoken::Algorithm::RS512
    | jsonwebtoken::Algorithm::PS256
    | jsonwebtoken::Algorithm::PS384
    | jsonwebtoken::Algorithm::PS512 => jsonwebtoken::EncodingKey::from_rsa_pem(value),
    // EC family
    jsonwebtoken::Algorithm::ES256 | jsonwebtoken::Algorithm::ES384 => {
      jsonwebtoken::EncodingKey::from_ec_pem(value)
    }

    // ED family
    jsonwebtoken::Algorithm::EdDSA => jsonwebtoken::EncodingKey::from_ed_pem(value),
  };

  encoding_key.map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))
}

pub struct SignTask {
  key: AsyncKeyInput,
  header: Header,
  claims: Claims,
}

impl SignTask {
  #[inline]
  pub fn sign(claims: &Claims, header: &Header, key: &[u8]) -> Result<String> {
    let header: &jsonwebtoken::Header = &header.into();
    let claims = &claims;
    let sign_key = &into_encoding_key(key, &header.alg)?;

    jsonwebtoken::encode(header, claims, sign_key)
      .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
  }
}

#[napi]
impl Task for SignTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    Self::sign(&self.claims, &self.header, self.key.as_ref())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    if let AsyncKeyInput::Buffer(buf) = &mut self.key {
      buf.unref(env)?;
    }
    Ok(())
  }
}

#[napi]
pub fn sign(
  claims: Claims,
  key: Either<String, JsBuffer>,
  header: Option<Header>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<SignTask>> {
  Ok(AsyncTask::with_optional_signal(
    SignTask {
      header: match header {
        Some(h) => h.merge(Header::default()),
        _ => Header::default(),
      },
      claims: claims.merge(Claims::default()),
      key: AsyncKeyInput::from_either(key)?,
    },
    abort_signal,
  ))
}

#[napi]
pub fn sign_sync(
  claims: Claims,
  key: Either<String, Buffer>,
  header: Option<Header>,
) -> Result<String> {
  let header = match header {
    Some(h) => h.merge(Header::default()),
    _ => Header::default(),
  };
  let claims = claims.merge(Claims::default());

  SignTask::sign(&claims, &header, key.as_ref())
}
