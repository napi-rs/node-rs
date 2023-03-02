use napi_derive::napi;
use serde_json::Number;

use crate::algorithm::Algorithm;

#[napi(object)]
#[derive(Default)]
pub struct Validation {
  /// If it contains a value, the validation will check that the `aud` field is a member of the
  /// audience provided and will error otherwise.
  ///
  /// Defaults to an empty collection.
  pub aud: Option<Vec<String>>,
  /// Which claims are required to be present before starting the validation.
  /// This does not interact with the various `validate_*`. If you remove `exp` from that list, you still need
  /// to set `validate_exp` to `false`.
  /// The only value that will be used are "exp", "nbf", "aud", "iss", "sub". Anything else will be ignored.
  ///
  /// Defaults to `exp`.
  pub required_spec_claims: Option<Vec<String>>,
  /// Add some leeway (in seconds) to the `exp` and `nbf` validation to
  /// account for clock skew.
  ///
  /// Defaults to `60`.
  pub leeway: Option<Number>,
  /// Whether to validate the `exp` field.
  ///
  /// Defaults to `true`.
  pub validate_exp: Option<bool>,
  /// Whether to validate the `nbf` field.
  ///
  /// It will return an error if the current timestamp is before the time in the `nbf` field.
  ///
  /// Defaults to `false`.
  pub validate_nbf: Option<bool>,
  /// If it contains a value, the validation will check that the `sub` field is the same as the
  /// one provided and will error otherwise.
  ///
  /// Turned off by default.
  pub sub: Option<String>,
  /// The algorithm used to verify the signature.
  ///
  /// Defaults to `HS256`.
  pub algorithms: Option<Vec<Algorithm>>,
  /// If it contains a value, the validation will check that the `iss` field is a member of the
  /// iss provided and will error otherwise.
  /// Use `set_issuer` to set it
  ///
  /// Defaults to an empty collection.
  pub iss: Option<Vec<String>>,
  /// Whether to validate the JWT signature.
  ///
  /// Defaults to `true`.
  pub validate_signature: Option<bool>,
}

impl From<&Validation> for jsonwebtoken::Validation {
  #[inline]
  fn from(value: &Validation) -> Self {
    let mut validation = Self::new(jsonwebtoken::Algorithm::HS256);

    if let Some(aud) = &value.aud {
      validation.set_audience(aud);
    }
    if let Some(required_spec_claims) = &value.required_spec_claims {
      validation.set_required_spec_claims(required_spec_claims);
    }
    if let Some(leeway) = value.leeway.clone().and_then(|l| l.as_u64()) {
      validation.leeway = leeway;
    }
    if let Some(validate_exp) = value.validate_exp {
      validation.validate_exp = validate_exp;
    }
    if let Some(validate_nbf) = value.validate_nbf {
      validation.validate_nbf = validate_nbf;
    }
    if let Some(sub) = &value.sub {
      validation.sub = Some(sub.to_string());
    }
    if let Some(algorithms) = &value.algorithms {
      validation.algorithms = algorithms.iter().map(|alg| alg.to_owned().into()).collect();
    }
    if let Some(iss) = &value.iss {
      validation.set_issuer(iss);
    }
    if let Some(false) = value.validate_signature {
      validation.insecure_disable_signature_validation()
    }

    validation
  }
}
