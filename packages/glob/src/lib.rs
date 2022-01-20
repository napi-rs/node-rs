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

  for entry in glob::glob(&pattern).expect("Failed to read Glob pattern") {
    match entry {
      Ok(path) => {
        results.push(path.to_str().unwrap().to_string());
      }

      // TODO: call callback with an error
      Err(e) => println!("{:?}", e),
    }
  }

  match callback(results) {
    Ok(_) => (),
    // TODO: handle failing to call the callback!
    Err(e) => println!("{:?}", e),
  }
}
