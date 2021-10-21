/// Explicit extern crate to use allocator.
extern crate global_alloc;

use napi::*;
use napi_derive::*;
use xxhash_rust::{xxh32, xxh64};

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
  exports.create_named_method("xxh32", xxh32)?;
  exports.create_named_method("xxh64", xxh64)?;

  let xxh32_class = env.define_class(
    "Xxh32",
    xxh32_constructor,
    &[
      Property::new(&env, "update")?.with_method(update_xxh32),
      Property::new(&env, "digest")?.with_method(digest_xxh32),
    ],
  )?;
  let xxh64_class = env.define_class(
    "Xxh64",
    xxh64_constructor,
    &[
      Property::new(&env, "update")?.with_method(update_xxh64),
      Property::new(&env, "digest")?.with_method(digest_xxh64),
    ],
  )?;
  exports.set_named_property("Xxh32", xxh32_class)?;
  exports.set_named_property("Xxh64", xxh64_class)?;
  Ok(())
}

#[js_function(2)]
fn xxh32(ctx: CallContext) -> Result<JsNumber> {
  let input = ctx.get::<JsBuffer>(0)?.into_value()?;
  let seed = ctx.get::<JsNumber>(1)?.get_uint32()?;
  ctx
    .env
    .create_uint32(xxhash_rust::xxh32::xxh32(input.as_ref(), seed))
}

#[js_function(1)]
fn xxh32_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let mut this = ctx.this_unchecked::<JsObject>();
  let seed = if ctx.length == 1 {
    ctx.get::<JsNumber>(0)?.get_uint32()?
  } else {
    0
  };
  let native = xxh32::Xxh32::new(seed);
  ctx.env.wrap(&mut this, native)?;
  ctx.env.get_undefined()
}

#[js_function(1)]
fn update_xxh32(ctx: CallContext) -> Result<JsObject> {
  let this = ctx.this_unchecked::<JsObject>();
  let native = ctx.env.unwrap::<xxh32::Xxh32>(&this)?;
  let input = ctx.get::<JsBuffer>(0)?.into_value()?;
  native.update(input.as_ref());
  Ok(this)
}

#[js_function]
fn digest_xxh32(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let native = ctx.env.unwrap::<xxh32::Xxh32>(&this)?;
  ctx.env.create_uint32(native.digest())
}

#[js_function(2)]
fn xxh64(ctx: CallContext) -> Result<JsBigint> {
  let input = ctx.get::<JsBuffer>(0)?.into_value()?;
  let (seed, _) = ctx.get::<JsBigint>(1)?.get_u64()?;
  ctx
    .env
    .create_bigint_from_u64(xxhash_rust::xxh64::xxh64(input.as_ref(), seed))
}

#[js_function(1)]
fn xxh64_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let mut this = ctx.this_unchecked::<JsObject>();
  let seed = if ctx.length == 1 {
    ctx.get::<JsBigint>(0)?.get_u64()?.0
  } else {
    0
  };
  let native = xxh64::Xxh64::new(seed);
  ctx.env.wrap(&mut this, native)?;
  ctx.env.get_undefined()
}

#[js_function(1)]
fn update_xxh64(ctx: CallContext) -> Result<JsObject> {
  let this = ctx.this_unchecked::<JsObject>();
  let native = ctx.env.unwrap::<xxh64::Xxh64>(&this)?;
  let input = ctx.get::<JsBuffer>(0)?.into_value()?;
  native.update(input.as_ref());
  Ok(this)
}

#[js_function]
fn digest_xxh64(ctx: CallContext) -> Result<JsBigint> {
  let this = ctx.this_unchecked::<JsObject>();
  let native = ctx.env.unwrap::<xxh64::Xxh64>(&this)?;
  ctx.env.create_bigint_from_u64(native.digest())
}
