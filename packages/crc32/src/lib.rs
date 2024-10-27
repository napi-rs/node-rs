#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi::bindgen_prelude::*;
use napi_derive::*;

#[napi(js_name = "crc32c")]
#[inline]
pub fn crc32c(input: Either<&[u8], String>, initial_state: Option<u32>) -> u32 {
  crc32c_append(initial_state.unwrap_or(0), input.as_ref())
}

#[napi]
#[inline]
pub fn crc32(input: Either<&[u8], String>, initial_state: Option<u32>) -> u32 {
  let mut hasher = Hasher::new_with_initial(initial_state.unwrap_or(0));
  hasher.update(input.as_ref());
  hasher.finalize()
}
