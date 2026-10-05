use base64::engine::Engine;
use napi::{Env, Result, Task};
use napi_derive::napi;
use rand::TryRng;
use rand::rngs::SysRng;

use bcrypt::Version;

// One getrandom syscall amortized over POOL_BYTES / 16 salts; the pool always
// holds raw OS entropy, never expanded by a userspace PRNG.
const POOL_BYTES: usize = 256;

thread_local! {
  static ENTROPY_POOL: std::cell::RefCell<([u8; POOL_BYTES], usize)> =
    const { std::cell::RefCell::new(([0; POOL_BYTES], POOL_BYTES)) };
}

#[inline]
pub(crate) fn gen_salt() -> [u8; 16] {
  ENTROPY_POOL.with(|pool| {
    let mut pool = pool.borrow_mut();
    let (bytes, offset) = &mut *pool;
    if *offset > POOL_BYTES - 16 {
      SysRng
        .try_fill_bytes(bytes)
        .expect("OS entropy source is unavailable");
      *offset = 0;
    }
    let salt: [u8; 16] = bytes[*offset..*offset + 16].try_into().unwrap();
    *offset += 16;
    salt
  })
}

#[inline]
pub(crate) fn format_salt(rounds: u32, version: &Version, salt: &[u8; 16]) -> String {
  let marker = match version {
    Version::TwoA => 'a',
    Version::TwoX => 'x',
    Version::TwoY => 'y',
    Version::TwoB => 'b',
  };
  let mut out = String::with_capacity(29);
  out.push_str("$2");
  out.push(marker);
  out.push('$');
  out.push((b'0' + (rounds / 10) as u8) as char);
  out.push((b'0' + (rounds % 10) as u8) as char);
  out.push('$');
  salt_engine().encode_string(salt, &mut out);
  out
}

// GeneralPurpose::new re-runs alphabet/config validation per construction; the
// engine is immutable, so build it once.
pub(crate) fn salt_engine() -> &'static base64::engine::general_purpose::GeneralPurpose {
  static ENGINE: std::sync::LazyLock<base64::engine::general_purpose::GeneralPurpose> =
    std::sync::LazyLock::new(|| {
      base64::engine::general_purpose::GeneralPurpose::new(
        &base64::alphabet::BCRYPT,
        base64::engine::general_purpose::NO_PAD,
      )
    });
  &ENGINE
}

pub struct SaltTask {
  pub(crate) round: u32,
  pub(crate) version: Version,
}

#[napi]
impl Task for SaltTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    let random = gen_salt();
    Ok(format_salt(self.round, &self.version, &random))
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
