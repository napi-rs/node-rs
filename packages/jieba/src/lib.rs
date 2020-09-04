#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

use jieba_rs::{Jieba, KeywordExtract, TFIDF};
use napi::{
  CallContext, Env, Error, JsBoolean, JsBuffer, JsNumber, JsObject, JsString, JsUndefined, Module,
  Result, Status,
};
use once_cell::sync::OnceCell;
use std::convert::TryInto;
use std::str;

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

register_module!(test_module, init);

static JIEBA: OnceCell<Jieba> = OnceCell::new();

fn init(module: &mut Module) -> Result<()> {
  module.create_named_method("load", load)?;
  module.create_named_method("loadDict", load_dict)?;
  module.create_named_method("cut", cut)?;
  module.create_named_method("cutAll", cut_all)?;
  module.create_named_method("cutForSearch", cut_for_search)?;
  module.create_named_method("tag", tag)?;
  module.create_named_method("extract", extract)?;
  Ok(())
}

#[js_function]
fn load(ctx: CallContext) -> Result<JsUndefined> {
  assert_not_init(&ctx.env)?;
  let _ = JIEBA.get_or_init(|| Jieba::new());
  ctx.env.get_undefined()
}

#[js_function(1)]
fn load_dict(ctx: CallContext) -> Result<JsUndefined> {
  assert_not_init(&ctx.env)?;
  let dict = ctx.get::<JsBuffer>(0)?;
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
fn cut(ctx: CallContext) -> Result<JsString> {
  let sentence = ctx.get::<JsBuffer>(0)?;
  let hmm = ctx
    .get::<JsBoolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted = jieba.cut(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value()?,
  );

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(1)]
fn cut_all(ctx: CallContext) -> Result<JsString> {
  let sentence = ctx.get::<JsBuffer>(0)?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted =
    jieba.cut_all(str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?);

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(2)]
fn cut_for_search(ctx: CallContext) -> Result<JsString> {
  let sentence = ctx.get::<JsBuffer>(0)?;
  let hmm = ctx
    .get::<JsBoolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let cutted = jieba.cut_for_search(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value()?,
  );

  let output = cutted.join(",");

  ctx.env.create_string(output.as_str())
}

#[js_function(1)]
fn tag(ctx: CallContext) -> Result<JsString> {
  let sentence = ctx.get::<JsBuffer>(0)?;
  let hmm = ctx
    .get::<JsBoolean>(1)
    .or_else(|_| ctx.env.get_boolean(false))?;
  let jieba = JIEBA.get_or_init(|| Jieba::new());
  let tagged = jieba.tag(
    str::from_utf8(&sentence).map_err(|_| Error::from_status(Status::InvalidArg))?,
    hmm.get_value()?,
  );

  let mut buf = vec![];

  for tag in tagged {
    buf.push(format!("{}|{}", tag.tag, tag.word));
  }

  ctx.env.create_string(buf.join(",").as_str())
}

#[js_function(3)]
fn extract(ctx: CallContext) -> Result<JsObject> {
  let sentence = ctx.get::<JsBuffer>(0)?;
  let topn = ctx.get::<JsNumber>(1)?;
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
  let mut js_tags = ctx.env.create_array_with_length(tags.len())?;

  for (index, t) in tags.iter().enumerate() {
    let mut tag_value = ctx.env.create_object()?;
    tag_value.set_named_property("keyword", ctx.env.create_string(t.keyword.as_str())?)?;
    tag_value.set_named_property("weight", ctx.env.create_double(t.weight)?)?;
    js_tags.set_element(index as _, tag_value)?;
  }

  Ok(js_tags)
}

#[js_function]
fn insert_word(ctx: CallContext) -> Result<JsUndefined> {
  ctx.env.get_undefined()
}
