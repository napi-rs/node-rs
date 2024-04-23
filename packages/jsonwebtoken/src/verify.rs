use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::{claims::Claims, validation::Validation};

pub enum AsyncKeyInput {
  String(String),
  Buffer(Uint8Array),
}

impl AsyncKeyInput {
  #[inline]
  pub fn from_either(input: Either<String, Uint8Array>) -> Result<Self> {
    match input {
      Either::A(s) => Ok(Self::String(s)),
      Either::B(b) => Ok(Self::Buffer(b)),
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
fn into_decoding_key(
  value: &[u8],
  alg: &jsonwebtoken::Algorithm,
) -> Result<jsonwebtoken::DecodingKey> {
  let encoding_key = match alg {
    // HMAC family
    jsonwebtoken::Algorithm::HS256
    | jsonwebtoken::Algorithm::HS384
    | jsonwebtoken::Algorithm::HS512 => Ok(jsonwebtoken::DecodingKey::from_secret(value)),
    // RSA family
    jsonwebtoken::Algorithm::RS256
    | jsonwebtoken::Algorithm::RS384
    | jsonwebtoken::Algorithm::RS512
    | jsonwebtoken::Algorithm::PS256
    | jsonwebtoken::Algorithm::PS384
    | jsonwebtoken::Algorithm::PS512 => jsonwebtoken::DecodingKey::from_rsa_pem(value),
    // EC family
    jsonwebtoken::Algorithm::ES256 | jsonwebtoken::Algorithm::ES384 => {
      jsonwebtoken::DecodingKey::from_ec_pem(value)
    }

    // ED family
    jsonwebtoken::Algorithm::EdDSA => jsonwebtoken::DecodingKey::from_ed_pem(value),
  };

  encoding_key.map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))
}

pub struct VerifyTask {
  token: String,
  key: AsyncKeyInput,
  validation: Validation,
}

impl VerifyTask {
  pub fn verify(token: &str, key: &[u8], validation: &Validation) -> Result<Claims> {
    let validation: &jsonwebtoken::Validation = &validation.into();

    let first_alg = validation.algorithms.first().ok_or(Error::new(
      Status::InvalidArg,
      "Validation `algorithms` should contain at least one valid algorithm".to_string(),
    ))?;
    let verify_key = &into_decoding_key(key, first_alg)?;

    jsonwebtoken::decode(token, verify_key, validation)
      .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
      .map(|token_data| token_data.claims)
  }
}

#[napi]
impl Task for VerifyTask {
  type Output = Claims;
  type JsValue = Claims;

  fn compute(&mut self) -> Result<Self::Output> {
    VerifyTask::verify(&self.token, self.key.as_ref(), &self.validation)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(ts_return_type = "Promise<{ [key: string]: any }>")]
pub fn verify(
  token: String,
  key: Either<String, Uint8Array>,
  validation: Option<Validation>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<VerifyTask>> {
  Ok(AsyncTask::with_optional_signal(
    VerifyTask {
      token,
      key: AsyncKeyInput::from_either(key)?,
      validation: validation.unwrap_or_default(),
    },
    abort_signal,
  ))
}

#[napi(ts_return_type = "{ [key: string]: any }")]
pub fn verify_sync(
  token: String,
  key: Either<String, &[u8]>,
  validation: Option<Validation>,
) -> Result<Claims> {
  let validation = validation.unwrap_or_default();
  VerifyTask::verify(&token, key.as_ref(), &validation)
}
