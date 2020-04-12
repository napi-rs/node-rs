extern crate napi_build;

use std::env;

fn main() {
  use napi_build::setup;

  setup();

  env::set_var("CARGO_RUSTC_FLAGS", "-C target-cpu=native");
}
