#![deny(clippy::all)]
#![allow(clippy::nonstandard_macro_braces)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use std::str;

use jieba_rs::{Jieba, KeywordExtract, TFIDF};
use napi::bindgen_prelude::*;
use napi_derive::*;
use once_cell::sync::OnceCell;

static JIEBA: OnceCell<Jieba> = OnceCell::new();
static TFIDF_INSTANCE: OnceCell<TFIDF> = OnceCell::new();

#[napi]
pub fn load() -> Result<()> {
  assert_not_init()?;
  let _ = JIEBA.get_or_init(Jieba::new);
  Ok(())
}

#[napi]
pub fn load_dict(dict: &[u8]) -> Result<()> {
  assert_not_init()?;
  let mut readable_dict: &[u8] = dict;
  JIEBA.get_or_init(|| {
    let mut jieba = Jieba::new();
    jieba
      .load_dict(&mut readable_dict)
      .map_err(|_| {
        Error::from_reason("Load dict failed".to_owned());
      })
      .unwrap();
    jieba
  });
  Ok(())
}

#[inline]
fn assert_not_init() -> Result<()> {
  if JIEBA.get().is_some() {
    Err(Error::from_reason(
      "Jieba was loaded, could not load again".to_owned(),
    ))
  } else {
    Ok(())
  }
}

#[napi(ts_return_type = "string[]")]
pub fn cut(env: Env, sentence: Either<String, &[u8]>, hmm: Option<bool>) -> Result<Array> {
  let hmm = hmm.unwrap_or(false);
  let jieba = JIEBA.get_or_init(Jieba::new);
  let cutted = jieba.cut(
    match &sentence {
      Either::A(s) => s.as_str(),
      Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
    },
    hmm,
  );
  Array::from_vec(&env, cutted)
}

#[napi(ts_return_type = "string[]")]
pub fn cut_all(env: Env, sentence: Either<String, &[u8]>) -> Result<Array> {
  let jieba = JIEBA.get_or_init(Jieba::new);
  let cutted = jieba.cut_all(match &sentence {
    Either::A(s) => s.as_str(),
    Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
  });

  Array::from_vec(&env, cutted)
}

#[napi(ts_return_type = "string[]")]
pub fn cut_for_search(
  env: Env,
  sentence: Either<String, &[u8]>,
  hmm: Option<bool>,
) -> Result<Array> {
  let hmm = hmm.unwrap_or(false);
  let jieba = JIEBA.get_or_init(Jieba::new);
  let cutted = jieba.cut_for_search(
    match &sentence {
      Either::A(s) => s.as_str(),
      Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
    },
    hmm,
  );

  Array::from_vec(&env, cutted)
}

#[napi(object)]
pub struct TaggedWord {
  pub tag: String,
  pub word: String,
}

#[napi]
pub fn tag(sentence: Either<String, &[u8]>, hmm: Option<bool>) -> Result<Vec<TaggedWord>> {
  let jieba = JIEBA.get_or_init(Jieba::new);
  let tagged = jieba.tag(
    match &sentence {
      Either::A(s) => s.as_str(),
      Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
    },
    hmm.unwrap_or(false),
  );

  Ok(
    tagged
      .iter()
      .map(|t| TaggedWord {
        tag: t.tag.to_owned(),
        word: t.word.to_owned(),
      })
      .collect(),
  )
}

#[napi(object)]
pub struct Keyword {
  pub keyword: String,
  pub weight: f64,
}

#[napi]
pub fn extract(
  sentence: Either<String, &[u8]>,
  topn: u32,
  allowed_pos: Option<String>,
) -> Result<Vec<Keyword>> {
  let allowed_pos_string = allowed_pos.unwrap_or_else(|| "".to_owned());

  let allowed_pos: Vec<String> = if allowed_pos_string.is_empty() {
    vec![]
  } else {
    allowed_pos_string
      .split(',')
      .map(|s| s.to_owned())
      .collect()
  };

  let keyword_extractor = TFIDF_INSTANCE.get_or_init(|| {
    let jieba = JIEBA.get_or_init(Jieba::new);
    TFIDF::new_with_jieba(jieba)
  });

  let tags = keyword_extractor.extract_tags(
    match &sentence {
      Either::A(s) => s.as_str(),
      Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
    },
    topn as usize,
    allowed_pos,
  );

  Ok(
    tags
      .into_iter()
      .map(|tag| Keyword {
        keyword: tag.keyword,
        weight: tag.weight,
      })
      .collect::<Vec<Keyword>>(),
  )
}

#[napi(js_name = "loadTFIDFDict")]
pub fn load_tfidf_dict(dict: &[u8]) -> Result<()> {
  let mut readable_dict: &[u8] = dict;
  if TFIDF_INSTANCE.get().is_some() {
    return Err(Error::new(
      Status::GenericFailure,
      "TFIDF has loaded, can not load dict again".to_owned(),
    ));
  }
  TFIDF_INSTANCE.get_or_try_init(|| {
    let jieba = JIEBA.get_or_init(Jieba::new);
    let mut tfidf = TFIDF::new_with_jieba(jieba);
    tfidf
      .load_dict(&mut readable_dict)
      .map(|_| tfidf)
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))
  })?;
  Ok(())
}
