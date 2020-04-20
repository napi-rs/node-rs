extern crate napi_build;

use std::env;

fn main() {
  napi_build::setup();

  env::set_var("RUSTFLAGS", "-C target-feature=+sse4.2");
}
