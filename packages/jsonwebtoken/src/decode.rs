use napi_derive::napi;

use crate::header::Header;

#[napi]
pub fn decode_header(token: String) -> Header {
  let result = jsonwebtoken::decode_header(&token);

  result.unwrap().into()
}
