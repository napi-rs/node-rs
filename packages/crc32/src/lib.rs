#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use crate::crc32::{crc32c as native_crc32c, crc32c_append};
use crc32fast::Hasher;
use napi::{CallContext, JsBuffer, JsNumber, Module, Result};
use std::convert::TryInto;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod bytes;
mod crc32;
mod crc32_table;

register_module!(crc32, init);

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("crc32c", crc32c)?;
  module.create_named_method("crc32", crc32)?;
  Ok(())
}

#[js_function(2)]
fn crc32c(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?;
  let init_state = ctx.get::<JsNumber>(1);
  let result = if init_state.is_ok() {
    crc32c_append(&input_data, init_state?.try_into()?)
  } else {
    native_crc32c(&input_data)
  };
  ctx.env.create_uint32(result)
}

#[js_function(2)]
fn crc32(ctx: CallContext) -> Result<JsNumber> {
  let input_data = ctx.get::<JsBuffer>(0)?;
  let init_state = ctx.get::<JsNumber>(1);
  let mut hasher = if init_state.is_ok() {
    Hasher::new_with_initial(init_state?.try_into()?)
  } else {
    Hasher::new()
  };
  hasher.update(&input_data);
  ctx.env.create_uint32(hasher.finalize())
}
