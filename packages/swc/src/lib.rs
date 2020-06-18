#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use napi::{Buffer, CallContext, Env, JsString, Object, Result, Value};
use transform_task::TransformTask;

mod transform_task;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

register_module!(test_module, init);

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_property(
    env.create_string("transformSync")?,
    env.create_function("transformSync", transform_sync)?,
  )?;

  exports.set_property(
    env.create_string("transform")?,
    env.create_function("transform", transform)?,
  )?;
  Ok(())
}

#[js_function(1)]
fn transform_sync(ctx: CallContext) -> Result<Value<JsString>> {
  ctx
    .env
    .create_string(TransformTask::perform(ctx.get::<Buffer>(0)?)?.as_str())
}

#[js_function(1)]
fn transform(ctx: CallContext) -> Result<Value<Object>> {
  let task = TransformTask::new(ctx.get::<Buffer>(0)?);
  ctx.env.spawn(task)
}
