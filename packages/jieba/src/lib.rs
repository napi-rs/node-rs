#[macro_use]
extern crate napi_rs as napi;
#[macro_use]
extern crate napi_rs_derive;

use jieba_rs::{Jieba, KeywordExtract, TFIDF};
use napi::{Buffer, CallContext, Env, Error, Object, Result, Status, Undefined, Value};
use napi_rs::Boolean;
use napi_rs::JsString;
use napi_rs::Number;
use once_cell::sync::OnceCell;
use std::convert::TryInto;
use std::str;

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

register_module!(test_module, init);

static JIEBA: OnceCell<Jieba> = OnceCell::new();

fn init(env: &Env, exports: &mut Value<Object>) -> Result<()> {
  exports.set_property(
    env.create_string("load")?,
    env.create_function("load", load)?,
  )?;

  exports.set_property(
    env.create_string("loadDict")?,
    env.create_function("loadDict", load_dict)?,
  )?;

  exports.set_property(env.create_string("cut")?, env.create_function("cut", cut)?)?;

  exports.set_property(
    env.create_string("cutAll")?,
    env.create_function("cutAll", cut_all)?,
  )?;

  exports.set_property(
    env.create_string("cutForSearch")?,
    env.create_function("cutForSearch", cut_for_search)?,
  )?;

  exports.set_property(env.create_string("tag")?, env.create_function("tag", tag)?)?;

  exports.set_property(
    env.create_string("extract")?,
    env.create_function("extract", extract)?,
  )?;

  Ok(())
}

#[js_function]
fn load(ctx: CallContext) -> Result<Value<Undefined>> {
  assert_not_init(&ctx.env)?;
  let _ = JIEBA.get_or_init(|| Jieba::new());
  ctx.env.get_undefined()
}

#[js_function(1)]
fn load_dict(ctx: CallContext) -> Result<Value<Undefined>> {
  assert_not_init(&ctx.env)?;
  let dict = ctx.get::<Buffer>(0)?;
  let mut readable_dict: &[u8] = &dict;
  JIEBA.get_or_init(|| {
    let mut jieba = Jieba::new();
    jieba
      .load_dict(&mut readable_dict)
      .map_err(|_| {
        ctx.env.throw_error("Load dict failed").unwrap();
      })
      .unwrap();
    jieba
  });
  ctx.env.get_undefined()
}

#[inline]
fn assert_not_init(env: &Env) -> Result<()> {
  if JIEBA.get().is_some() {
    env.throw_error("Jieba was loaded, could not load again")
  } else {
    Ok(())
  }
}

#[js_function(2)]
fn cut(ctx: CallContext) -> Result<Value<JsString>> {
  let sentence = ctx.get::<Buffer>(0)?;
  let hmm = ctx
    .get::<Boolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted = jieba.cut(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value(),
  );

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(1)]
fn cut_all(ctx: CallContext) -> Result<Value<JsString>> {
  let sentence = ctx.get::<Buffer>(0)?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted =
    jieba.cut_all(str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?);

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(2)]
fn cut_for_search(ctx: CallContext) -> Result<Value<JsString>> {
  let sentence = ctx.get::<Buffer>(0)?;
  let hmm = ctx
    .get::<Boolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted = jieba.cut_for_search(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value(),
  );

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(1)]
fn tag(ctx: CallContext) -> Result<Value<JsString>> {
  let sentence = ctx.get::<Buffer>(0)?;
  let hmm = ctx
    .get::<Boolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let tagged = jieba.tag(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value(),
  );

  let mut buf = vec![];

  for tag in tagged {
    buf.push(format!("{}|{}", tag.tag, tag.word));
  }

  ctx.env.create_string(buf.join(",").as_str())
}

#[js_function(3)]
fn extract(ctx: CallContext) -> Result<Value<JsString>> {
  let sentence = ctx.get::<Buffer>(0)?;
  let topn = ctx.get::<Number>(1)?;
  let allowed_pos = ctx
    .get::<JsString>(2)
    .or_else(|_| ctx.env.create_string(""))?;

  let allowed_pos_str = allowed_pos.as_str()?;

  let allowed_pos: Vec<String> = if allowed_pos_str.len() == 0 {
    vec![]
  } else {
    allowed_pos_str.split(",").map(|s| s.to_owned()).collect()
  };

  let jieba = JIEBA.get_or_init(|| Jieba::new());

  let keyword_extractor = TFIDF::new_with_jieba(&jieba);

  let topn: usize = topn.try_into()?;

  let tags = keyword_extractor.extract_tags(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    topn,
    allowed_pos,
  );

  ctx.env.create_string(tags.join(",").as_str())
}

#[js_function]
fn insert_word(ctx: CallContext) -> Result<Value<Undefined>> {
  ctx.env.get_undefined()
}
