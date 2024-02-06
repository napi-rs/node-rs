#![deny(clippy::all)]
#![allow(dead_code)]

/// Explicit extern crate to use allocator.
extern crate global_alloc;

mod algorithm;
mod claims;
mod header;
mod sign;
mod validation;
mod verify;

pub use algorithm::Algorithm;
pub use claims::Claims;
pub use header::Header;
pub use sign::{sign, sign_sync};
pub use validation::Validation;
pub use verify::{verify, verify_sync};
