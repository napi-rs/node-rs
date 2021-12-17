#![deny(clippy::all)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi::bindgen_prelude::{Buffer, Either};
use napi_derive::*;

#[napi(js_name = "crc32c")]
pub fn crc32c(input: Either<String, Buffer>, initial_state: Option<u32>) -> u32 {
  crc32c_append(
    initial_state.unwrap_or(0),
    match &input {
      Either::A(s) => s.as_bytes(),
      Either::B(b) => b.as_ref(),
    },
  )
}

#[napi]
pub fn crc32(input_data: Either<String, Buffer>, initial_state: Option<u32>) -> u32 {
  let mut hasher = Hasher::new_with_initial(initial_state.unwrap_or(0));
  hasher.update(match &input_data {
    Either::A(s) => s.as_bytes(),
    Either::B(b) => b.as_ref(),
  });
  hasher.finalize()
}
