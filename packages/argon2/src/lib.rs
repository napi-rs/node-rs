#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use napi::{Env, Object, Result, Value};

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

register_module!(argon2, init);

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  Ok(())
}
