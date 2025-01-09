use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize};
use titan_core::{FromRequestParts, Respondable};
use titan_http::{body::Body, Parts, Response};

#[derive(Deserialize)]
pub struct Query<T = HashMap<String, String>>(pub T);

pub enum QueryParseError {
  QueryParamsDoesntExist,
  ParserError(String),
}

impl Respondable for QueryParseError {
  fn respond(self) -> Response {
    let body = match self {
      Self::QueryParamsDoesntExist => {
        Body::from("Required Query parameters where not provided")
      }
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
      return Err(QueryParseError::QueryParamsDoesntExist);
    };

    match serde_urlencoded::from_str::<T>(query_str) {
      Ok(nice) => Ok(Query(nice)),
      Err(err) => {
        println!("error: {err}");
        Err(QueryParseError::ParserError(err.to_string()))
      }
    }
  }
}
