#![deny(clippy::all)]

use glob::{glob_with, MatchOptions};
use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask},
  Env, Result, Task,
};
use napi_derive::napi;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi(object)]
#[derive(Default, Clone, Copy)]
pub struct GlobOptions {
  /// Whether or not patterns should be matched in a case-sensitive manner.
  /// This currently only considers upper/lower case relationships between
  /// ASCII characters, but in future this might be extended to work with
  /// Unicode.
  pub case_sensitive: Option<bool>,

  /// Whether or not path-component separator characters (e.g. `/` on
  /// Posix) must be matched by a literal `/`, rather than by `*` or `?` or
  /// `[...]`.
  pub require_literal_separator: Option<bool>,

  /// Whether or not paths that contain components that start with a `.`
  /// will require that `.` appears literally in the pattern; `*`, `?`, `**`,
  /// or `[...]` will not match. This is useful because such files are
  /// conventionally considered hidden on Unix systems and it might be
  /// desirable to skip them when listing files.
  pub require_literal_leading_dot: Option<bool>,
}

fn get_match_option(options: Option<GlobOptions>) -> MatchOptions {
  return {
    MatchOptions {
      case_sensitive: options
        .as_ref()
        .map(|o| o.case_sensitive.unwrap_or(false))
        .unwrap_or(false),
      require_literal_separator: options
        .as_ref()
        .map(|o| o.require_literal_separator.unwrap_or(false))
        .unwrap_or(false),
      require_literal_leading_dot: options
        .as_ref()
        .map(|o| o.require_literal_leading_dot.unwrap_or(false))
        .unwrap_or(false),
    }
  };
}

/// Run glob task sync
#[napi]
pub fn glob_sync(pattern: String, options: Option<GlobOptions>) -> Result<Vec<String>> {
  let mut results = Vec::new();
  let match_options = get_match_option(options);

  let paths = glob_with(&pattern, match_options).map_err(|err| {
    napi::Error::new(
      napi::Status::GenericFailure,
      format!("Globing the pattern {} failed. {}", pattern, err),
    )
  })?;

  for path_result in paths {
    let path = path_result.map_err(|err| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Globing the pattern {} failed. {}", pattern, err),
      )
    })?;
    results.push(path.into_os_string().into_string().unwrap());
  }

  return Ok(results);
}

pub struct GlobTask {
  pub(crate) pattern: String,
  pub(crate) options: Option<GlobOptions>,
}

#[napi]
impl Task for GlobTask {
  type Output = Vec<String>;
  type JsValue = Vec<String>;

  fn compute(&mut self) -> Result<Self::Output> {
    let results = glob_sync(self.pattern.clone(), self.options.clone()).map_err(|err| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!(
          "Globing the pattern {} failed. {}",
          self.pattern.clone(),
          err
        ),
      )
    })?;

    return Ok(results);
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

/// Run glob async
#[napi]
pub fn glob(
  pattern: String,
  options: Option<GlobOptions>,
  abort_signal: Option<AbortSignal>,
) -> Result<AsyncTask<GlobTask>> {
  Ok(AsyncTask::with_optional_signal(
    GlobTask {
      pattern: pattern,
      options: options,
    },
    abort_signal,
  ))
}
