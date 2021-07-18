#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

#[macro_use]
extern crate napi_derive;

use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi::{CallContext, JsBuffer, JsNumber, JsObject, Result};
use std::convert::TryInto;

#[cfg(all(
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("crc32c", crc32c)?;
  exports.create_named_method("crc32", crc32)?;
  Ok(())
}

#[js_function(2)]
fn crc32c(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?.into_value()?;
  let init_state = ctx.get::<JsNumber>(1)?;
  let result = crc32c_append(init_state.try_into()?, &input_data);
  ctx.env.create_uint32(result)
}

#[js_function(2)]
fn crc32(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?.into_value()?;
  let init_state = ctx.get::<JsNumber>(1)?;
  let mut hasher = Hasher::new_with_initial(init_state.try_into()?);
  hasher.update(&input_data);
  ctx.env.create_uint32(hasher.finalize())
}
