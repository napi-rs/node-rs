#![deny(clippy::all)]
use glob::{glob_with, MatchOptions};
use napi::Result;
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
#[derive(Default)]
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

#[napi]
pub fn glob(pattern: String, options: Option<GlobOptions>) -> Result<Vec<String>> {
  let mut results = Vec::new();
  let glob_options = MatchOptions {
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
  };

  for entry in glob_with(&pattern, glob_options).expect("Failed to read Glob pattern") {
    match entry {
      Ok(path) => {
        results.push(path.to_str().unwrap().to_string());
      }

      Err(e) => println!("{:?}", e),
    }
  }

  return Ok(results);
}
