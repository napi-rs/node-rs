/// Explicit extern crate to use allocator.
extern crate global_alloc;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use xxhash_rust::{xxh32, xxh64};

#[napi]
fn xxh32(input: Either<String, Buffer>, seed: Option<u32>) -> u32 {
  let seed = seed.unwrap_or(0);
  xxhash_rust::xxh32::xxh32(
    match &input {
      Either::A(s) => s.as_bytes(),
      Either::B(b) => b.as_ref(),
    },
    seed,
  )
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
  pub fn update(&mut self, input: Either<String, Buffer>) -> &Self {
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
fn xxh64(input: Either<String, Buffer>, seed: Option<BigInt>) -> u64 {
  let seed = seed.map(|b| b.get_u64().1).unwrap_or(0);
  xxhash_rust::xxh64::xxh64(
    match &input {
      Either::A(s) => s.as_bytes(),
      Either::B(b) => b.as_ref(),
    },
    seed,
  )
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
  pub fn update(&mut self, input: Either<String, Buffer>) -> &Self {
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
  pub fn xxh64(input: Either<String, Buffer>, seed: Option<BigInt>) -> u64 {
    let seed = seed.map(|b| b.get_u64().1).unwrap_or(0);
    match input {
      Either::A(s) => xxh3::xxh3_64_with_seed(s.as_bytes(), seed),
      Either::B(b) => xxh3::xxh3_64_with_seed(b.as_ref(), seed),
    }
  }

  #[napi]
  pub fn xxh64_with_secret(input: Either<String, Buffer>, secret: Buffer) -> u64 {
    match input {
      Either::A(s) => xxh3::xxh3_64_with_secret(s.as_bytes(), secret.as_ref()),
      Either::B(b) => xxh3::xxh3_64_with_secret(b.as_ref(), secret.as_ref()),
    }
  }

  #[napi]
  pub fn xxh128(input: Either<String, Buffer>, seed: Option<BigInt>) -> u128 {
    let seed = seed.map(|b| b.get_u64().1).unwrap_or(0);
    xxh3::xxh3_128_with_seed(
      match &input {
        Either::A(s) => s.as_bytes(),
        Either::B(b) => b.as_ref(),
      },
      seed,
    )
  }

  #[napi]
  pub fn xxh128_with_secret(input: Either<String, Buffer>, secret: Buffer) -> u128 {
    xxh3::xxh3_128_with_secret(
      match &input {
        Either::A(s) => s.as_bytes(),
        Either::B(b) => b.as_ref(),
      },
      secret.as_ref(),
    )
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
    pub fn with_secret(secret: Buffer) -> Self {
      let mut sec = [0u8; 192];
      sec.copy_from_slice(secret.as_ref());
      Self {
        inner: xxh3::Xxh3::with_secret(sec),
      }
    }

    #[napi]
    pub fn update(&mut self, input: Either<String, Buffer>) -> &Self {
      self.inner.update(match &input {
        Either::A(s) => s.as_bytes(),
        Either::B(b) => b.as_ref(),
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
