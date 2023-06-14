use napi_derive::napi;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<Map<String, Value>>,
  // Recipient for which the JWT is intended
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aud: Option<String>,
  // Time after which the JWT expires (as UTC timestamp, seconds from epoch time)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub exp: Option<Number>,
  // Time at which the JWT was issued (as UTC timestamp, seconds from epoch time)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iat: Option<Number>,
  // Issuer of JWT
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iss: Option<String>,
  // [JWT id] Unique identifier
  #[serde(skip_serializing_if = "Option::is_none")]
  pub jti: Option<String>,
  // [not-before-time] Time before which the JWT must not be accepted for processing (as UTC timestamp, seconds from epoch time)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nbf: Option<Number>,
  // Subject of JWT (the user)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub: Option<String>,
}

impl Claims {
  #[inline]
  pub fn merge(self, other: Self) -> Self {
    Self {
      data: self.data,
      aud: self.aud.or(other.aud),
      exp: self.exp.or(other.exp),
      iat: self.iat.or(other.iat),
      iss: self.iss.or(other.iss),
      jti: self.jti.or(other.jti),
      nbf: self.nbf.or(other.nbf),
      sub: self.sub.or(other.sub),
    }
  }
}

impl Default for Claims {
  #[inline]
  fn default() -> Self {
    Self {
      data: Some(Map::new()),
      aud: None,
      exp: None,
      iat: Some(jsonwebtoken::get_current_timestamp().into()),
      iss: None,
      jti: None,
      nbf: None,
      sub: None,
    }
  }
}
