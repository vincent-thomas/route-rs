use std::convert::Infallible;

use serde::Deserialize;
use titan_core::{FromRequestParts, Respondable};
use titan_http::{Parts, Response};

#[derive(Deserialize)]
pub struct Path(pub String);

pub struct PathError;

impl Respondable for PathError {
  fn respond(self) -> Response {
    unreachable!()
  }
}

impl FromRequestParts for Path {
  type Error = Infallible;
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    Ok(Path(parts.uri.path().to_string()))
  }
}
