use napi_derive::napi;

use crate::algorithm::Algorithm;

#[napi(object)]
pub struct Header {
  /// The algorithm used
  ///
  /// Defined in [RFC7515#4.1.1](https://tools.ietf.org/html/rfc7515#section-4.1.1).
  /// Default to `HS256`
  pub algorithm: Option<Algorithm>,

  /// Content type
  ///
  /// Defined in [RFC7519#5.2](https://tools.ietf.org/html/rfc7519#section-5.2).
  pub content_type: Option<String>,

  /// JSON Key URL
  ///
  /// Defined in [RFC7515#4.1.2](https://tools.ietf.org/html/rfc7515#section-4.1.2).
  pub json_key_url: Option<String>,

  /// JSON Web Key
  ///
  /// Defined in [RFC7515#4.1.3](https://tools.ietf.org/html/rfc7515#section-4.1.3).
  // TODO: support jwk
  // pub jwk: Option<Jwk>,

  /// Key ID
  ///
  /// Defined in [RFC7515#4.1.4](https://tools.ietf.org/html/rfc7515#section-4.1.4).
  pub key_id: Option<String>,

  /// X.509 URL
  ///
  /// Defined in [RFC7515#4.1.5](https://tools.ietf.org/html/rfc7515#section-4.1.5).
  pub x5_url: Option<String>,

  /// X.509 certificate chain. A Vec of base64 encoded ASN.1 DER certificates.
  ///
  /// Defined in [RFC7515#4.1.6](https://tools.ietf.org/html/rfc7515#section-4.1.6).
  pub x5_cert_chain: Option<Vec<String>>,

  /// X.509 SHA1 certificate thumbprint
  ///
  /// Defined in [RFC7515#4.1.7](https://tools.ietf.org/html/rfc7515#section-4.1.7).
  pub x5_cert_thumbprint: Option<String>,

  /// X.509 SHA256 certificate thumbprint
  ///
  /// Defined in [RFC7515#4.1.8](https://tools.ietf.org/html/rfc7515#section-4.1.8).
  ///
  /// This will be serialized/deserialized as "x5t#S256", as defined by the RFC.
  pub x5t_s256_cert_thumbprint: Option<String>,
}

impl From<&Header> for jsonwebtoken::Header {
  #[inline]
  fn from(value: &Header) -> Self {
    jsonwebtoken::Header {
      typ: Some(String::from("JWT")),
      alg: value.algorithm.unwrap_or(Algorithm::ES256).into(),
      cty: value.content_type.clone(),
      jku: value.json_key_url.clone(),
      kid: value.key_id.clone(),
      x5u: value.x5_url.clone(),
      x5c: value.x5_cert_chain.clone(),
      x5t: value.x5_cert_thumbprint.clone(),
      x5t_s256: value.x5t_s256_cert_thumbprint.clone(),
      // TODO: support jwk
      jwk: None,
    }
  }
}

impl Into<jsonwebtoken::Header> for Header {
  #[inline]
  fn into(value: &jsonwebtoken::Header) -> Self {
    Header {
      algorithm: value.alg,
      content_type: value.cty.clone(),
    }
  }
}

impl Header {
  #[inline]
  pub fn merge(self, other: Self) -> Self {
    Self {
      algorithm: self.algorithm.or(other.algorithm),
      content_type: self.content_type.or(other.content_type),
      json_key_url: self.json_key_url.or(other.json_key_url),
      key_id: self.key_id.or(other.key_id),
      x5_url: self.x5_url.or(other.x5_url),
      x5_cert_chain: self.x5_cert_chain.or(other.x5_cert_chain),
      x5_cert_thumbprint: self.x5_cert_thumbprint.or(other.x5_cert_thumbprint),
      x5t_s256_cert_thumbprint: self
        .x5t_s256_cert_thumbprint
        .or(other.x5t_s256_cert_thumbprint),
    }
  }
}

impl Default for Header {
  #[inline]
  fn default() -> Self {
    Self {
      algorithm: Some(Algorithm::HS256),
      content_type: None,
      json_key_url: None,
      key_id: None,
      x5_url: None,
      x5_cert_chain: None,
      x5_cert_thumbprint: None,
      x5t_s256_cert_thumbprint: None,
    }
  }
}
