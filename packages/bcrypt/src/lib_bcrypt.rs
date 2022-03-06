use core::{fmt, str::FromStr};
use getrandom::getrandom;

use crate::errors::{BcryptError, BcryptResult};

// Cost constants
pub(crate) const MIN_COST: u32 = 4;
pub(crate) const MAX_COST: u32 = 31;

#[derive(Debug, PartialEq)]
/// A bcrypt hash result before concatenating
pub struct HashParts {
  cost: u32,
  salt: String,
  hash: String,
}

/// BCrypt hash version
/// https://en.wikipedia.org/wiki/Bcrypt#Versioning_history
#[derive(Clone, Copy)]
pub enum Version {
  TwoA,
  TwoX,
  TwoY,
  TwoB,
}

impl FromStr for Version {
  type Err = BcryptError;

  fn from_str(s: &str) -> Result<Version, Self::Err> {
    if s == "2a" {
      return Ok(Version::TwoA);
    }
    if s == "2x" {
      return Ok(Version::TwoX);
    }
    if s == "2y" {
      return Ok(Version::TwoY);
    }
    if s == "2b" {
      return Ok(Version::TwoB);
    }
    Err(BcryptError::InvalidVersion(s.to_owned()))
  }
}

impl HashParts {
  /// Creates the bcrypt hash string from all its parts
  fn format(self) -> String {
    self.format_for_version(Version::TwoB)
  }

  /// Get the bcrypt hash cost
  pub fn get_cost(&self) -> u32 {
    self.cost
  }

  /// Get the bcrypt hash salt
  pub fn get_salt(&self) -> String {
    self.salt.clone()
  }

  /// Creates the bcrypt hash string from all its part, allowing to customize the version.
  pub fn format_for_version(&self, version: Version) -> String {
    // Cost need to have a length of 2 so padding with a 0 if cost < 10
    format!("${}${:02}${}{}", version, self.cost, self.salt, self.hash)
  }
}

impl FromStr for HashParts {
  type Err = BcryptError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    split_hash(s)
  }
}

impl ToString for HashParts {
  fn to_string(&self) -> String {
    self.format_for_version(Version::TwoY)
  }
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      Version::TwoA => "2a",
      Version::TwoB => "2b",
      Version::TwoX => "2x",
      Version::TwoY => "2y",
    };
    write!(f, "{}", str)
  }
}

/// The main meat: actually does the hashing and does some verification with
/// the cost to ensure it's a correct one
fn _hash_password(password: &[u8], cost: u32, salt: &[u8]) -> BcryptResult<HashParts> {
  if !(MIN_COST..=MAX_COST).contains(&cost) {
    return Err(BcryptError::CostNotAllowed(cost));
  }

  // Output is 24
  let mut output = [0u8; 24];
  // Passwords need to be null terminated
  let mut vec = Vec::with_capacity(password.len() + 1);
  vec.extend_from_slice(password);
  vec.push(0);
  // We only consider the first 72 chars; truncate if necessary.
  // `bcrypt` below will panic if len > 72
  let truncated = if vec.len() > 72 { &vec[..72] } else { &vec };

  bcrypt::bcrypt(cost, salt, truncated, &mut output);

  Ok(HashParts {
    cost,
    salt: base64::encode_config(salt, base64::BCRYPT),
    hash: base64::encode_config(&output[..23], base64::BCRYPT), // remember to remove the last byte
  })
}

/// Takes a full hash and split it into 3 parts:
/// cost, salt and hash
fn split_hash(hash: &str) -> BcryptResult<HashParts> {
  let mut parts = HashParts {
    cost: 0,
    salt: "".to_string(),
    hash: "".to_string(),
  };

  // Should be [prefix, cost, hash]
  let raw_parts: Vec<_> = hash.split('$').filter(|s| !s.is_empty()).collect();

  if raw_parts.len() != 3 {
    return Err(BcryptError::InvalidHash(hash.to_string()));
  }

  if raw_parts[0] != "2y" && raw_parts[0] != "2b" && raw_parts[0] != "2a" && raw_parts[0] != "2x" {
    return Err(BcryptError::InvalidPrefix(raw_parts[0].to_string()));
  }

  if let Ok(c) = raw_parts[1].parse::<u32>() {
    parts.cost = c;
  } else {
    return Err(BcryptError::InvalidCost(raw_parts[1].to_string()));
  }

  if raw_parts[2].len() == 53 && raw_parts[2].is_char_boundary(22) {
    parts.salt = raw_parts[2][..22].chars().collect();
    parts.hash = raw_parts[2][22..].chars().collect();
  } else {
    return Err(BcryptError::InvalidHash(hash.to_string()));
  }

  Ok(parts)
}

/// Generates a password hash using the cost given.
/// The salt is generated randomly using the OS randomness
pub fn hash<P: AsRef<[u8]>>(password: P, cost: u32) -> BcryptResult<String> {
  hash_with_result(password, cost).map(|r| r.format())
}

/// Generates a password hash using the cost given.
/// The salt is generated randomly using the OS randomness.
/// The function returns a result structure and allows to format the hash in different versions.
pub fn hash_with_result<P: AsRef<[u8]>>(password: P, cost: u32) -> BcryptResult<HashParts> {
  let salt = {
    let mut s = [0u8; 16];
    getrandom(&mut s).map(|_| s)
  }?;

  _hash_password(password.as_ref(), cost, salt.as_ref())
}

/// Generates a password given a hash and a cost.
/// The function returns a result structure and allows to format the hash in different versions.
pub fn hash_with_salt<P: AsRef<[u8]>>(
  password: P,
  cost: u32,
  salt: &[u8],
) -> BcryptResult<HashParts> {
  _hash_password(password.as_ref(), cost, salt)
}

/// Verify that a password is equivalent to the hash provided
pub fn verify<P: AsRef<[u8]>>(password: P, hash: &str) -> BcryptResult<bool> {
  let parts = split_hash(hash)?;
  let salt = base64::decode_config(&parts.salt, base64::BCRYPT)?;
  let generated = _hash_password(password.as_ref(), parts.cost, &salt)?;
  let source_decoded = base64::decode_config(&parts.hash, base64::BCRYPT)?;
  let generated_decoded = base64::decode_config(&generated.hash, base64::BCRYPT)?;
  if source_decoded.len() != generated_decoded.len() {
    return Ok(false);
  }

  let mut diff = 0;
  for (a, b) in source_decoded.into_iter().zip(generated_decoded) {
    diff |= a ^ b;
  }

  Ok(diff == 0)
}

#[inline]
pub fn gen_salt() -> BcryptResult<[u8; 16]> {
  let mut s = [0u8; 16];
  getrandom(&mut s).map(|_| s).map_err(|err| err.into())
}

#[inline]
pub fn format_salt(rounds: u32, version: Version, salt: &[u8; 16]) -> String {
  format!(
    "${}${:0>2}${}",
    version,
    rounds,
    base64::encode_config(salt, base64::BCRYPT)
  )
}

#[cfg(test)]
mod tests {
  use super::{
    _hash_password, hash, hash_with_salt, split_hash, verify, BcryptError, BcryptResult, HashParts,
    Version,
  };
  use crate::DEFAULT_COST;
  use core::iter;
  use core::str::FromStr;
  use quickcheck::{quickcheck, TestResult};

  #[test]
  fn can_split_hash() {
    let hash = "$2y$12$L6Bc/AlTQHyd9liGgGEZyOFLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u";
    let output = split_hash(hash).unwrap();
    let expected = HashParts {
      cost: 12,
      salt: "L6Bc/AlTQHyd9liGgGEZyO".to_string(),
      hash: "FLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u".to_string(),
    };
    assert_eq!(output, expected);
  }

  #[test]
  fn can_output_cost_and_salt_from_parsed_hash() {
    let hash = "$2y$12$L6Bc/AlTQHyd9liGgGEZyOFLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u";
    let parsed = HashParts::from_str(hash).unwrap();
    assert_eq!(parsed.get_cost(), 12);
    assert_eq!(parsed.get_salt(), "L6Bc/AlTQHyd9liGgGEZyO".to_string());
  }

  #[test]
  fn returns_an_error_if_a_parsed_hash_is_baddly_formated() {
    let hash1 = "$2y$12$L6Bc/AlTQHyd9lGEZyOFLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u";
    assert!(HashParts::from_str(hash1).is_err());

    let hash2 = "!2y$12$L6Bc/AlTQHyd9liGgGEZyOFLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u";
    assert!(HashParts::from_str(hash2).is_err());

    let hash3 = "$2y$-12$L6Bc/AlTQHyd9liGgGEZyOFLPHNgyxeEPfgYfBCVxJ7JIlwxyVU3u";
    assert!(HashParts::from_str(hash3).is_err());
  }

  #[test]
  fn can_verify_hash_generated_from_some_online_tool() {
    let hash = "$2a$04$UuTkLRZZ6QofpDOlMz32MuuxEHA43WOemOYHPz6.SjsVsyO1tDU96";
    assert!(verify("password", hash).unwrap());
  }

  #[test]
  fn can_verify_hash_generated_from_python() {
    let hash = "$2b$04$EGdrhbKUv8Oc9vGiXX0HQOxSg445d458Muh7DAHskb6QbtCvdxcie";
    assert!(verify("correctbatteryhorsestapler", hash).unwrap());
  }

  #[test]
  fn can_verify_hash_generated_from_node() {
    let hash = "$2a$04$n4Uy0eSnMfvnESYL.bLwuuj0U/ETSsoTpRT9GVk5bektyVVa5xnIi";
    assert!(verify("correctbatteryhorsestapler", hash).unwrap());
  }

  #[test]
  fn can_verify_hash_generated_from_go() {
    /*
        package main
        import (
            "io"
            "os"
            "golang.org/x/crypto/bcrypt"
        )
        func main() {
            buf, err := io.ReadAll(os.Stdin)
            if err != nil {
                panic(err)
            }
            out, err := bcrypt.GenerateFromPassword(buf, bcrypt.MinCost)
            if err != nil {
                panic(err)
            }
            os.Stdout.Write(out)
            os.Stdout.Write([]byte("\n"))
        }
    */
    let binary_input = vec![
      29, 225, 195, 167, 223, 236, 85, 195, 114, 227, 7, 0, 209, 239, 189, 24, 51, 105, 124, 168,
      151, 75, 144, 64, 198, 197, 196, 4, 241, 97, 110, 135,
    ];
    let hash = "$2a$04$tjARW6ZON3PhrAIRW2LG/u9aDw5eFdstYLR8nFCNaOQmsH9XD23w.";
    assert!(verify(binary_input, hash).unwrap());
  }

  #[test]
  fn a_wrong_password_is_false() {
    let hash = "$2b$04$EGdrhbKUv8Oc9vGiXX0HQOxSg445d458Muh7DAHskb6QbtCvdxcie";
    assert!(!verify("wrong", hash).unwrap());
  }

  #[test]
  fn errors_with_invalid_hash() {
    // there is another $ in the hash part
    let hash = "$2a$04$n4Uy0eSnMfvnESYL.bLwuuj0U/ETSsoTpRT9GVk$5bektyVVa5xnIi";
    assert!(verify("correctbatteryhorsestapler", hash).is_err());
  }

  #[test]
  fn errors_with_non_number_cost() {
    // the cost is not a number
    let hash = "$2a$ab$n4Uy0eSnMfvnESYL.bLwuuj0U/ETSsoTpRT9GVk$5bektyVVa5xnIi";
    assert!(verify("correctbatteryhorsestapler", hash).is_err());
  }

  #[test]
  fn errors_with_a_hash_too_long() {
    // the cost is not a number
    let hash = "$2a$04$n4Uy0eSnMfvnESYL.bLwuuj0U/ETSsoTpRT9GVk$5bektyVVa5xnIerererereri";
    assert!(verify("correctbatteryhorsestapler", hash).is_err());
  }

  #[test]
  fn can_verify_own_generated() {
    let hashed = hash("hunter2", 4).unwrap();
    assert_eq!(true, verify("hunter2", &hashed).unwrap());
  }

  #[test]
  fn long_passwords_truncate_correctly() {
    // produced with python -c 'import bcrypt; bcrypt.hashpw(b"x"*100, b"$2a$05$...............................")'
    let hash = "$2a$05$......................YgIDy4hFBdVlc/6LHnD9mX488r9cLd2";
    assert!(verify(iter::repeat("x").take(100).collect::<String>(), hash).unwrap());
  }

  #[test]
  fn generate_versions() {
    let password = "hunter2".as_bytes();
    let salt = vec![0; 16];
    let result = _hash_password(password, DEFAULT_COST, salt.as_slice()).unwrap();
    assert_eq!(
      "$2a$12$......................21jzCB1r6pN6rp5O2Ev0ejjTAboskKm",
      result.format_for_version(Version::TwoA)
    );
    assert_eq!(
      "$2b$12$......................21jzCB1r6pN6rp5O2Ev0ejjTAboskKm",
      result.format_for_version(Version::TwoB)
    );
    assert_eq!(
      "$2x$12$......................21jzCB1r6pN6rp5O2Ev0ejjTAboskKm",
      result.format_for_version(Version::TwoX)
    );
    assert_eq!(
      "$2y$12$......................21jzCB1r6pN6rp5O2Ev0ejjTAboskKm",
      result.format_for_version(Version::TwoY)
    );
    let hash = result.to_string();
    assert_eq!(true, verify("hunter2", &hash).unwrap());
  }

  #[test]
  fn allow_null_bytes() {
    // hash p1, check the hash against p2:
    fn hash_and_check(p1: &[u8], p2: &[u8]) -> Result<bool, BcryptError> {
      let fast_cost = 4;
      match hash(p1, fast_cost) {
        Ok(s) => verify(p2, &s),
        Err(e) => Err(e),
      }
    }
    fn assert_valid_password(p1: &[u8], p2: &[u8], expected: bool) {
      match hash_and_check(p1, p2) {
        Ok(checked) => {
          if checked != expected {
            panic!(
              "checked {:?} against {:?}, incorrect result {}",
              p1, p2, checked
            )
          }
        }
        Err(e) => panic!("error evaluating password: {} for {:?}.", e, p1),
      }
    }

    // bcrypt should consider all of these distinct:
    let test_passwords = vec![
      "\0",
      "passw0rd\0",
      "password\0with tail",
      "\0passw0rd",
      "a",
      "a\0",
      "a\0b\0",
    ];

    for (i, p1) in test_passwords.iter().enumerate() {
      for (j, p2) in test_passwords.iter().enumerate() {
        assert_valid_password(p1.as_bytes(), p2.as_bytes(), i == j);
      }
    }

    // this is a quirk of the bcrypt algorithm: passwords that are entirely null
    // bytes hash to the same value, even if they are different lengths:
    assert_valid_password("\0\0\0\0\0\0\0\0".as_bytes(), "\0".as_bytes(), true);
  }

  #[test]
  fn hash_with_fixed_salt() {
    let salt = vec![
      38, 113, 212, 141, 108, 213, 195, 166, 201, 38, 20, 13, 47, 40, 104, 18,
    ];
    let hashed = hash_with_salt("My S3cre7 P@55w0rd!", 5, &salt)
      .unwrap()
      .to_string();
    assert_eq!(
      "$2y$05$HlFShUxTu4ZHHfOLJwfmCeDj/kuKFKboanXtDJXxCC7aIPTUgxNDe",
      &hashed
    );
  }

  quickcheck! {
      fn can_verify_arbitrary_own_generated(pass: Vec<u8>) -> BcryptResult<bool> {
          let mut pass = pass;
          pass.retain(|&b| b != 0);
          let hashed = hash(&pass, 4)?;
          verify(pass, &hashed)
      }

      fn doesnt_verify_different_passwords(a: Vec<u8>, b: Vec<u8>) -> BcryptResult<TestResult> {
          let mut a = a;
          a.retain(|&b| b != 0);
          let mut b = b;
          b.retain(|&b| b != 0);
          if a == b {
              return Ok(TestResult::discard());
          }
          let hashed = hash(a, 4)?;
          Ok(TestResult::from_bool(!verify(b, &hashed)?))
      }
  }

  #[test]
  fn does_no_error_on_char_boundary_splitting() {
    // Just checks that it does not panic
    let _ = verify(
      &[],
      "2a$$$0$OOOOOOOOOOOOOOOOOOOOO£OOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
    );
  }
}
