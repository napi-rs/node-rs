#![deny(clippy::all)]

use glob;
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

#[napi]
pub fn glob_pattern<T: Fn(Vec<String>) -> Result<()>>(pattern: String, callback: T) {
  let mut results = Vec::new();

  let _glob_results = glob::glob(&pattern).map(|paths| {
    let _paths_rsults = paths.map(|path| {
      results.push(
        path
          .unwrap()
          .to_owned()
          .into_os_string()
          .into_string()
          .unwrap(),
      );
    });
  });
  // .collect();

  let _callback_results = callback(results);
}
