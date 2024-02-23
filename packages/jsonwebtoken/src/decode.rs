use napi::{bindgen_prelude::*};
use napi_derive::napi;

use crate::{header::Header};

#[napi]
pub fn decode_header(
    token: &str
) -> Header {
    let result = jsonwebtoken::decode_header(&token);

    let header = Header::from(result.unwrap());
}