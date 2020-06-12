#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use crate::crc32::{crc32c as native_crc32c, crc32c_append};
use crc32fast::Hasher;
use napi::{Buffer, CallContext, Env, Number, Object, Result, Value};
use std::convert::TryInto;

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod bytes;
mod crc32;
mod crc32_table;

register_module!(test_module, init);

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_named_property("crc32c", env.create_function("crc32c", crc32c)?)?;
  exports.set_named_property("crc32", env.create_function("crc32", crc32)?)?;
  Ok(())
}

#[js_function(2)]
fn crc32c(ctx: CallContext) -> Result<Value<Number>> {
  let input_data = ctx.get::<Buffer>(0)?;
  let init_state = ctx.get::<Number>(1);
  let result = if init_state.is_ok() {
    crc32c_append(&input_data, init_state?.try_into()?)
  } else {
    native_crc32c(&input_data)
  };
  ctx.env.create_uint32(result)
}

#[js_function(2)]
fn crc32(ctx: CallContext) -> Result<Value<Number>> {
  let input_data = ctx.get::<Buffer>(0)?;
  let init_state = ctx.get::<Number>(1);
  let mut hasher = if init_state.is_ok() {
    Hasher::new_with_initial(init_state?.try_into()?)
  } else {
    Hasher::new()
  };
  hasher.update(&input_data);
  ctx.env.create_uint32(hasher.finalize())
}
