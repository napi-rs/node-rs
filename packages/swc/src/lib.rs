#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use napi::{Buffer, CallContext, Env, JsString, Object, Result, Value};
use uglify_task::UglifyTask;

mod uglify_task;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

register_module!(test_module, init);

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_property(
    env.create_string("uglifySync")?,
    env.create_function("uglifySync", uglify_sync)?,
  )?;

  exports.set_property(
    env.create_string("uglify")?,
    env.create_function("uglify", uglify)?,
  )?;
  Ok(())
}

#[js_function(1)]
fn uglify_sync(ctx: CallContext) -> Result<Value<JsString>> {
  ctx
    .env
    .create_string(UglifyTask::uglify(ctx.get::<Buffer>(0)?)?.as_str())
}

#[js_function(1)]
fn uglify(ctx: CallContext) -> Result<Value<Object>> {
  let task = UglifyTask::new(ctx.get::<Buffer>(0)?);
  ctx.env.spawn(task)
}
