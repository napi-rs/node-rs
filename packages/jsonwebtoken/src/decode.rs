use napi::{Error, Result, Status};
use napi_derive::napi;

use crate::header::Header;

#[napi]
pub fn decode_header(token: String) -> Result<Header> {
  jsonwebtoken::decode_header(&token)
    .map(Into::into)
    .map_err(|err| Error::new(Status::InvalidArg, format!("{err}")))
}
