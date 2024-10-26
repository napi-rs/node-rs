#![deny(clippy::all)]
#![allow(clippy::new_without_default)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use std::collections::BTreeSet;
use std::mem;
use std::str;
use std::sync::Arc;

use jieba_rs::KeywordExtract;
use napi::bindgen_prelude::*;
use napi_derive::*;

#[napi(object)]
pub struct TaggedWord {
  pub tag: String,
  pub word: String,
}

#[napi(object)]
pub struct Keyword {
  pub keyword: String,
  pub weight: f64,
}

#[napi(object)]
/// Creates a KeywordExtractConfig state that contains filter criteria as
/// well as segmentation configuration for use by keyword extraction
/// implementations.
pub struct KeywordExtractConfig {
  #[napi(ts_type = "Set<string> | undefined")]
  pub stop_words: Option<Object>,
  /// Any segments less than this length will not be considered a Keyword
  pub min_keyword_length: Option<u32>,
  /// If true, fall back to hmm model if segment cannot be found in the dictionary
  pub use_hmm: Option<bool>,
}

impl TryFrom<KeywordExtractConfig> for jieba_rs::KeywordExtractConfig {
  type Error = Error;

  fn try_from(config: KeywordExtractConfig) -> Result<Self> {
    let mut stop_words = BTreeSet::new();
    if let Some(sw) = config.stop_words.as_ref() {
      let iter_func: Function<'_, (), Object> = sw.get_named_property_unchecked("values")?;
      let iter = iter_func.apply(sw, ())?;
      while {
        let next_fn: Function<'_, (), Object> = iter.get_named_property_unchecked("next")?;
        let next = next_fn.apply(&iter, ())?;
        let done: bool = next.get_named_property_unchecked("done")?;
        if !done {
          let value: String = iter.get_named_property_unchecked("value")?;
          stop_words.insert(value);
        }
        !done
      } {}
      {}
    }

    jieba_rs::KeywordExtractConfig::builder()
      .set_stop_words(stop_words)
      .min_keyword_length(config.min_keyword_length.unwrap_or(2) as usize)
      .use_hmm(config.use_hmm.unwrap_or(false))
      .build()
      .map_err(|err| {
        Error::new(
          Status::InvalidArg,
          format!("Failed to create KeywordExtractConfig from JavaScript value: {err}"),
        )
      })
  }
}

#[napi]
#[repr(transparent)]
pub struct TfIdf(jieba_rs::TfIdf);

#[napi]
impl TfIdf {
  #[napi(factory)]
  pub fn with_dict(dict: &[u8]) -> Self {
    let mut dict = dict;
    TfIdf(jieba_rs::TfIdf::new(
      Some(&mut dict),
      jieba_rs::KeywordExtractConfig::default(),
    ))
  }

  #[napi(constructor)]
  /// Creates an TfIdf.
  pub fn new() -> Self {
    TfIdf(jieba_rs::TfIdf::new(
      Option::<&mut &[u8]>::None,
      jieba_rs::KeywordExtractConfig::default(),
    ))
  }

  #[napi]
  /// Merges entires from `dict` into the `idf_dict`.
  /// ```js
  /// import { Jieba, TfIdf } from '@node-rs/jieba';
  ///
  /// import { dict, idf } from '@node-rs/jieba/dict';
  ///
  /// // Create default Jieba instance
  /// const jieba = Jieba.withDict(dict);
  ///
  /// // Create TfIdf instance and load initial dictionary
  /// let initIdf = "生化学 13.900677652\n";
  /// const tfidf = new TfIdf();
  /// tfidf.loadDict(Buffer.from(initIdf));
  ///
  /// // Extract keywords with initial dictionary
  /// const text = "生化学不是光化学的,";
  /// const topK = jieba.extract(text, 3);
  /// // Result would be like:
  /// // [
  /// //   { keyword: '不是', weight: 4.6335592173333335 },
  /// //   { keyword: '光化学', weight: 4.6335592173333335 },
  /// //   { keyword: '生化学', weight: 4.6335592173333335 }
  /// // ]
  ///
  /// // Load new dictionary with different weights
  /// let newIdf = "光化学 99.123456789\n";
  /// tfidf.loadDict(Buffer.from(newIdf));
  ///
  /// // Extract keywords again with updated dictionary
  /// const newTopK = jieba.extract(text, 3);
  /// // Result would be like:
  /// // [
  /// //   { keyword: '不是', weight: 33.041152263 },
  /// //   { keyword: '光化学', weight: 33.041152263 },
  /// //   { keyword: '生化学', weight: 4.6335592173333335 }
  /// // ]
  /// ```
  pub fn load_dict(&mut self, dict: &[u8]) -> Result<()> {
    let mut readable_dict: &[u8] = dict;
    self
      .0
      .load_dict(&mut readable_dict)
      .map_err(|_| Error::from_reason("Load dict failed".to_owned()))
  }

  #[napi]
  pub fn set_config(&mut self, config: KeywordExtractConfig) -> Result<()> {
    mem::drop(mem::replace(self.0.config_mut(), config.try_into()?));
    Ok(())
  }

  #[napi]
  /// Uses TF-IDF algorithm to extract the `top_k` keywords from `sentence`.
  ///
  /// If `allowed_pos` is not empty, then only terms matching those parts if
  /// speech are considered.
  pub fn extract_keywords(
    &self,
    jieba: &Jieba,
    sentence: String,
    top_k: u32,
    allowed_pos: Option<Vec<String>>,
  ) -> Vec<Keyword> {
    self
      .0
      .extract_keywords(
        &jieba.0,
        &sentence,
        top_k as usize,
        allowed_pos.unwrap_or_default(),
      )
      .into_iter()
      .map(|k| Keyword {
        keyword: k.keyword,
        weight: k.weight,
      })
      .collect()
  }
}

#[napi]
#[repr(transparent)]
pub struct Jieba(Arc<jieba_rs::Jieba>);

#[napi]
impl Jieba {
  #[napi(constructor)]
  /// Create a new instance with empty dict
  pub fn new() -> Self {
    Jieba(Arc::new(jieba_rs::Jieba::empty()))
  }

  #[napi(factory)]
  /// Create a new instance with dict
  ///
  /// With the default dict, you can use `dict` from `@node-rs/jieba/dict`:
  /// ```js
  /// import { Jieba } from '@node-rs/jieba'
  /// import { dict } from '@node-rs/jieba/dict'
  ///
  /// const jieba = Jieba.withDict(dict)
  /// ```
  pub fn with_dict(dict: &[u8]) -> Result<Self> {
    let mut readable_dict: &[u8] = dict;
    let jieba = jieba_rs::Jieba::with_dict(&mut readable_dict)
      .map_err(|_| Error::from_reason("Load dict failed".to_owned()))?;
    Ok(Jieba(Arc::new(jieba)))
  }

  #[napi]
  /// Load dictionary after initialization
  pub fn load_dict(&mut self, dict: &[u8]) -> Result<()> {
    let mut readable_dict: &[u8] = dict;
    Arc::get_mut(&mut self.0)
      .expect("JavaScript is single-threaded")
      .load_dict(&mut readable_dict)
      .map_err(|_| Error::from_reason("Load dict failed".to_owned()))
  }

  #[napi(ts_return_type = "string[]")]
  /// Cut the input text
  ///
  /// ## Params
  ///
  /// `sentence`: input text
  ///
  /// `hmm`: enable HMM or not
  pub fn cut(
    &self,
    env: &Env,
    sentence: Either<String, &[u8]>,
    hmm: Option<bool>,
  ) -> Result<Array> {
    let hmm = hmm.unwrap_or(false);
    let cutted = self.0.cut(
      match &sentence {
        Either::A(s) => s.as_str(),
        Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
      },
      hmm,
    );
    Array::from_vec(env, cutted)
  }

  #[napi]
  /// Cut the input text asynchronously
  pub fn cut_async(
    &self,
    sentence: Either<String, Uint8Array>,
    hmm: Option<bool>,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<CutTask> {
    AsyncTask::with_optional_signal(
      CutTask {
        jieba: self.0.clone(),
        sentence,
        hmm,
      },
      signal,
    )
  }

  #[napi(ts_return_type = "string[]")]
  /// Cut the input text, return all possible words
  ///
  /// ## Params
  ///
  /// `sentence`: input text
  pub fn cut_all(&self, env: &Env, sentence: Either<String, &[u8]>) -> Result<Array> {
    let cutted = self.0.cut_all(match &sentence {
      Either::A(s) => s.as_str(),
      Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
    });

    Array::from_vec(env, cutted)
  }

  #[napi(ts_return_type = "string[]")]
  /// Cut the input text in search mode
  ///
  /// ## Params
  ///
  /// `sentence`: input text
  ///
  /// `hmm`: enable HMM or not
  pub fn cut_for_search(
    &self,
    env: &Env,
    sentence: Either<String, &[u8]>,
    hmm: Option<bool>,
  ) -> Result<Array> {
    let hmm = hmm.unwrap_or(false);
    let cutted = self.0.cut_for_search(
      match &sentence {
        Either::A(s) => s.as_str(),
        Either::B(b) => str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?,
      },
      hmm,
    );

    Array::from_vec(env, cutted)
  }

  #[napi]
  /// Tag the input text
  ///
  /// ## Params
  ///
  /// `sentence`: input text
  ///
  /// `hmm`: enable HMM or not
  pub fn tag(&self, sentence: Either<String, &[u8]>, hmm: Option<bool>) -> Vec<TaggedWord> {
    let tagged = self.0.tag(
      match &sentence {
        Either::A(s) => s.as_str(),
        Either::B(b) => str::from_utf8(b).unwrap(),
      },
      hmm.unwrap_or(false),
    );

    tagged
      .iter()
      .map(|t| TaggedWord {
        tag: t.tag.to_owned(),
        word: t.word.to_owned(),
      })
      .collect()
  }
}

pub struct CutTask {
  sentence: Either<String, Uint8Array>,
  hmm: Option<bool>,
  jieba: Arc<jieba_rs::Jieba>,
}

#[napi]
impl Task for CutTask {
  type Output = Vec<String>;
  type JsValue = Vec<String>;

  fn compute(&mut self) -> Result<Self::Output> {
    let hmm = self.hmm.unwrap_or(false);
    Ok(
      self
        .jieba
        .cut(
          match &self.sentence {
            Either::A(s) => s.as_str(),
            Either::B(b) => {
              str::from_utf8(b).map_err(|_| Error::from_status(Status::InvalidArg))?
            }
          },
          hmm,
        )
        .iter()
        .map(|s| s.to_string())
        .collect(),
    )
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
