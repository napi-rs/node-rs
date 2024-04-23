use napi_derive::napi;

#[napi(string_enum)]
pub enum Algorithm {
  /// HMAC using SHA-256
  HS256,
  /// HMAC using SHA-384
  HS384,
  /// HMAC using SHA-512
  HS512,
  /// ECDSA using SHA-256
  ES256,
  /// ECDSA using SHA-384
  ES384,
  /// RSASSA-PKCS1-v1_5 using SHA-256
  RS256,
  /// RSASSA-PKCS1-v1_5 using SHA-384
  RS384,
  /// RSASSA-PKCS1-v1_5 using SHA-512
  RS512,
  /// RSASSA-PSS using SHA-256
  PS256,
  /// RSASSA-PSS using SHA-384
  PS384,
  /// RSASSA-PSS using SHA-512
  PS512,
  /// Edwards-curve Digital Signature Algorithm (EdDSA)
  EdDSA,
}

impl From<Algorithm> for jsonwebtoken::Algorithm {
  #[inline]
  fn from(value: Algorithm) -> Self {
    match value {
      Algorithm::ES256 => jsonwebtoken::Algorithm::ES256,
      Algorithm::ES384 => jsonwebtoken::Algorithm::ES384,
      Algorithm::EdDSA => jsonwebtoken::Algorithm::EdDSA,
      Algorithm::HS256 => jsonwebtoken::Algorithm::HS256,
      Algorithm::HS384 => jsonwebtoken::Algorithm::HS384,
      Algorithm::HS512 => jsonwebtoken::Algorithm::HS512,
      Algorithm::PS256 => jsonwebtoken::Algorithm::PS256,
      Algorithm::PS384 => jsonwebtoken::Algorithm::PS384,
      Algorithm::PS512 => jsonwebtoken::Algorithm::PS512,
      Algorithm::RS256 => jsonwebtoken::Algorithm::RS256,
      Algorithm::RS384 => jsonwebtoken::Algorithm::RS384,
      Algorithm::RS512 => jsonwebtoken::Algorithm::RS512,
    }
  }
}

impl From<jsonwebtoken::Algorithm> for Algorithm {
  #[inline]
  fn from(value: jsonwebtoken::Algorithm) -> Self {
    match value {
      jsonwebtoken::Algorithm::ES256 => Algorithm::ES256,
      jsonwebtoken::Algorithm::ES384 => Algorithm::ES384,
      jsonwebtoken::Algorithm::EdDSA => Algorithm::EdDSA,
      jsonwebtoken::Algorithm::HS256 => Algorithm::HS256,
      jsonwebtoken::Algorithm::HS384 => Algorithm::HS384,
      jsonwebtoken::Algorithm::HS512 => Algorithm::HS512,
      jsonwebtoken::Algorithm::PS256 => Algorithm::PS256,
      jsonwebtoken::Algorithm::PS384 => Algorithm::PS384,
      jsonwebtoken::Algorithm::PS512 => Algorithm::PS512,
      jsonwebtoken::Algorithm::RS256 => Algorithm::RS256,
      jsonwebtoken::Algorithm::RS384 => Algorithm::RS384,
      jsonwebtoken::Algorithm::RS512 => Algorithm::RS512,
    }
  }
}

impl Default for Algorithm {
  fn default() -> Self {
    Self::HS256
  }
}
