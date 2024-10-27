#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use xxhash_rust::{xxh32, xxh64};

#[napi]
#[inline]
fn xxh32(input: Either<&[u8], String>, seed: Option<u32>) -> u32 {
  #[cfg(not(target_arch = "x86_64"))]
  {
    xxhash_rust::const_xxh32::xxh32(input.as_ref(), seed.unwrap_or(0))
  }
  #[cfg(target_arch = "x86_64")]
  {
    xxhash_rust::xxh32::xxh32(input.as_ref(), seed.unwrap_or(0))
  }
}

#[napi]
pub struct Xxh32 {
  inner: xxh32::Xxh32,
}

#[napi]
impl Xxh32 {
  #[napi(constructor)]
  pub fn new(seed: Option<u32>) -> Xxh32 {
    Xxh32 {
      inner: xxh32::Xxh32::new(seed.unwrap_or(0)),
    }
  }

  #[napi]
  pub fn update(&mut self, input: Either<String, &[u8]>) -> &Self {
    match input {
      Either::A(s) => self.inner.update(s.as_bytes()),
      Either::B(b) => self.inner.update(b.as_ref()),
    };
    self
  }

  #[napi]
  pub fn digest(&self) -> u32 {
    self.inner.digest()
  }

  #[napi]
  pub fn reset(&mut self, new_state: Option<u32>) {
    self.inner.reset(new_state.unwrap_or(0));
  }
}

#[napi]
#[inline]
fn xxh64(input: Either<&[u8], String>, seed: Option<BigInt>) -> u64 {
  xxhash_rust::xxh64::xxh64(input.as_ref(), seed.map(|b| b.get_u64().1).unwrap_or(0))
}

#[napi]
pub struct Xxh64 {
  inner: xxh64::Xxh64,
}

#[napi]
impl Xxh64 {
  #[napi(constructor)]
  pub fn new(seed: Option<BigInt>) -> Self {
    Self {
      inner: xxh64::Xxh64::new(seed.map(|b| b.get_u64().1).unwrap_or(0)),
    }
  }

  #[napi]
  pub fn update(&mut self, input: Either<String, &[u8]>) -> &Self {
    match input {
      Either::A(s) => self.inner.update(s.as_bytes()),
      Either::B(b) => self.inner.update(b.as_ref()),
    };
    self
  }

  #[napi]
  pub fn digest(&self) -> u64 {
    self.inner.digest()
  }

  #[napi]
  pub fn reset(&mut self, new_state: Option<BigInt>) {
    self
      .inner
      .reset(new_state.map(|b| b.get_u64().1).unwrap_or(0));
  }
}

#[napi(js_name = "xxh3")]
mod xxh3_js {
  use napi::bindgen_prelude::*;
  use xxhash_rust::xxh3;

  #[napi]
  #[inline]
  pub fn xxh64(input: Either<&[u8], String>, seed: Option<BigInt>) -> u64 {
    xxhash_rust::const_xxh3::xxh3_64_with_seed(
      input.as_ref(),
      seed.map(|b| b.get_u64().1).unwrap_or(0),
    )
  }

  #[napi]
  #[inline]
  pub fn xxh64_with_secret(input: Either<String, &[u8]>, secret: &[u8]) -> u64 {
    xxh3::xxh3_64_with_secret(input.as_ref(), secret)
  }

  #[napi]
  #[inline]
  pub fn xxh128(input: Either<String, &[u8]>, seed: Option<BigInt>) -> u128 {
    xxhash_rust::const_xxh3::xxh3_128_with_seed(
      input.as_ref(),
      seed.map(|b| b.get_u64().1).unwrap_or(0),
    )
  }

  #[napi]
  #[inline]
  pub fn xxh128_with_secret(input: Either<String, &[u8]>, secret: &[u8]) -> u128 {
    xxh3::xxh3_128_with_secret(input.as_ref(), secret)
  }

  #[napi]
  pub struct Xxh3 {
    inner: xxh3::Xxh3,
  }

  #[napi]
  impl Xxh3 {
    #[napi(factory)]
    pub fn with_seed(seed: Option<BigInt>) -> Self {
      Self {
        inner: xxh3::Xxh3::with_seed(seed.map(|b| b.get_u64().1).unwrap_or(0)),
      }
    }

    #[napi(factory)]
    pub fn with_secret(secret: &[u8]) -> Self {
      let mut sec = [0u8; 192];
      sec.copy_from_slice(secret.as_ref());
      Self {
        inner: xxh3::Xxh3::with_secret(sec),
      }
    }

    #[napi]
    pub fn update(&mut self, input: Either<String, &[u8]>) -> &Self {
      self.inner.update(match &input {
        Either::A(s) => s.as_bytes(),
        Either::B(b) => b,
      });
      self
    }

    #[napi]
    pub fn digest(&self) -> u64 {
      self.inner.digest()
    }

    #[napi]
    pub fn reset(&mut self) {
      self.inner.reset();
    }
  }
}
