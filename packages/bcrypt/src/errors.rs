use std::error;
use std::fmt;
use std::io;

/// Library generic result type.
pub type BcryptResult<T> = Result<T, BcryptError>;

#[derive(Debug)]
/// All the errors we can encounter while hashing/verifying
/// passwords
pub enum BcryptError {
  Io(io::Error),
  InvalidVersion(String),
  InvalidCost(String),
  InvalidPrefix(String),
  InvalidHash(String),
  Rand(rand::Error),
}

macro_rules! impl_from_error {
  ($f: ty, $e: expr) => {
    impl From<$f> for BcryptError {
      fn from(f: $f) -> BcryptError {
        $e(f)
      }
    }
  };
}

impl_from_error!(io::Error, BcryptError::Io);
impl_from_error!(rand::Error, BcryptError::Rand);

impl fmt::Display for BcryptError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      BcryptError::Io(ref err) => write!(f, "IO error: {}", err),
      BcryptError::InvalidCost(ref cost) => write!(f, "Invalid Cost: {}", cost),
      BcryptError::InvalidVersion(ref v) => write!(f, "Invalid version: {}", v),
      BcryptError::InvalidPrefix(ref prefix) => write!(f, "Invalid Prefix: {}", prefix),
      BcryptError::InvalidHash(ref hash) => write!(f, "Invalid hash: {}", hash),
      BcryptError::Rand(ref err) => write!(f, "Rand error: {}", err),
    }
  }
}

impl error::Error for BcryptError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      BcryptError::Io(ref err) => Some(err),
      BcryptError::InvalidCost(_)
      | BcryptError::InvalidVersion(_)
      | BcryptError::InvalidPrefix(_)
      | BcryptError::InvalidHash(_) => None,
      BcryptError::Rand(ref err) => Some(err),
    }
  }
}
