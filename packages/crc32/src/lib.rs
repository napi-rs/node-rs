#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi::{CallContext, JsBuffer, JsNumber, JsObject, Result};
use napi_derive::*;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("crc32c", crc32c)?;
  exports.create_named_method("crc32", crc32)?;
  Ok(())
}

#[js_function(2)]
fn crc32c(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?.into_value()?;
  let init_state = ctx.get::<JsNumber>(1)?.get_uint32()?;
  let result = crc32c_append(init_state, &input_data);
  ctx.env.create_uint32(result)
}

#[js_function(2)]
fn crc32(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?.into_value()?;
  let init_state = ctx.get::<JsNumber>(1)?.get_uint32()?;
  let mut hasher = Hasher::new_with_initial(init_state);
  hasher.update(&input_data);
  ctx.env.create_uint32(hasher.finalize())
}
