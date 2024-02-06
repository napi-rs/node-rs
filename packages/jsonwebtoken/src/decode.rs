use napi::{bindgen_prelude::*, JsBuffer, JsBufferValue, Ref};
use napi_derive::napi;

use crate::{decode::HeaderDecoded};

#[napi]
pub fn decode_header(
    token: &str
) -> Result<HeaderDecoded> {
    jsonwebtoken::decode_header(&token);
}