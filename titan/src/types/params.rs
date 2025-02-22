use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use titan_core::{FromRequestParts, Respondable};
use titan_http::{body::Body, Parts, Response};

#[derive(Deserialize)]
pub struct Params<T = HashMap<String, Value>>(pub T);

pub enum ParamParseError {
  InvalidFormat,
  ParserError(String),
}

impl Respondable for ParamParseError {
  fn respond(self) -> Response {
    let body = match self {
      Self::InvalidFormat => Body::from("Invalid Format"),
      Self::ParserError(err) => {
        Body::from(format!("Param Parsing Error: {err}"))
      }
    };
    Response::builder().status(400).body(body).unwrap()
  }
}

impl<T> FromRequestParts for Params<T>
where
  T: DeserializeOwned,
{
  type Error = ParamParseError;
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    let map = parts.extensions.get::<HashMap<String, Value>>();

    let value = serde_json::to_value(map.unwrap()).unwrap();

    match serde_json::from_value(value) {
      Ok(value) => Ok(value),
      Err(err) => Err(ParamParseError::ParserError(err.to_string())),
    }
  }
}
