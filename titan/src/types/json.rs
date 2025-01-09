use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use titan_http::{body::Body, Request, Response};

use crate::{FromRequest, Respondable};

use super::BodyParsingError;

#[derive(Clone)]
pub struct Json<T = HashMap<String, Value>>(pub T);

impl<T> FromRequest for Json<T>
where
  T: DeserializeOwned,
{
  type Error = BodyParsingError;
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let content_type = req.headers().get("content-type");
    let body = req.body();
    if content_type.is_none()
      || content_type.is_some_and(|v| v != "application/json")
    {
      Err(BodyParsingError::ContentTypeInvalid)
    } else if body.is_empty() {
      Err(BodyParsingError::NoBody)
    } else {
      let json = serde_json::from_slice(body)
        .map_err(|x| BodyParsingError::ParsingError(x.to_string()))?;
      Ok(Json(json))
    }
  }
}

impl<W: Serialize> Serialize for Json<W> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<S: Serialize> Respondable for Json<S> {
  fn respond(self) -> Response {
    let body = serde_json::to_string(&self.0).unwrap();
    let body_len = body.len();
    let mut res = Response::new(Body::from(body));
    let headers = res.headers_mut();

    headers.insert(
      titan_http::header::CONTENT_TYPE,
      titan_http::mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );

    headers.insert(titan_http::header::CONTENT_LENGTH, body_len.into());
    res
  }
}
