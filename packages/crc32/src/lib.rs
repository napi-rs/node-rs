#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use crate::crc32::crc32c_append;
use crc32fast::Hasher;
use napi::{CallContext, JsBuffer, JsNumber, JsObject, Result};
use std::convert::TryInto;

#[cfg(all(
  unix,
  not(target_env = "musl"),
  not(target_os = "freebsd"),
  not(target_arch = "arm"),
  not(target_arch = "aarch64")
))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod bytes;
mod crc32;
mod crc32_table;

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
  let result = crc32c_append(&input_data, init_state.try_into()?);
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
