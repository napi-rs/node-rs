#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi::bindgen_prelude::*;
use napi_derive::*;

#[napi(js_name = "crc32c")]
pub fn crc32c(input: Either<String, &[u8]>, initial_state: Option<u32>) -> Result<u32> {
  Ok(match input {
    Either::A(s) => crc32c_append(initial_state.unwrap_or(0), s.as_bytes()),
    Either::B(b) => crc32c_append(initial_state.unwrap_or(0), b),
  })
}

#[napi]
pub fn crc32(input: Either<String, &[u8]>, initial_state: Option<u32>) -> Result<u32> {
  let mut hasher = Hasher::new_with_initial(initial_state.unwrap_or(0));
  match input {
    Either::A(s) => {
      hasher.update(s.as_bytes());
    }
    Either::B(b) => {
      hasher.update(b);
    }
  };
  Ok(hasher.finalize())
}
