use std::collections::HashMap;

use route_core::{FromRequestParts, Respondable};
use route_http::{body::Body, request::Parts, response::Response};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize)]
pub struct Query<T = HashMap<String, String>>(pub T);

pub enum QueryParseError {
  InvalidFormat,
  ParserError(String),
}

impl Respondable for QueryParseError {
  fn respond(self) -> Response {
    let body = match self {
      Self::InvalidFormat => Body::from("Invalid Format"),
      Self::ParserError(err) => {
        Body::from(format!("Query Parsing Error: {err}"))
      }
    };
    Response::builder().status(400).body(body).unwrap()
  }
}

impl<T> FromRequestParts for Query<T>
where
  T: DeserializeOwned,
{
  type Error = QueryParseError;
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    let Some(query_str) = parts.uri.query() else {
      return Err(QueryParseError::InvalidFormat);
    };

    match serde_urlencoded::from_str::<T>(query_str) {
      Ok(nice) => Ok(Query(nice)),
      Err(err) => Err(QueryParseError::ParserError(err.to_string())),
    }
  }
}
