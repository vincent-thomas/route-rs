use route_http::{request::Request, response::Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{FromRequest, respond::Respondable};

#[derive(Clone)]
pub struct Json<T>(pub T);

enum JsonParseError {
    NoBody,
    ContentTypeInvalid,
    ParseError(serde_json::Error)
}

impl<T> FromRequest for Json<T>
where
  T: DeserializeOwned,
{
  type Error = JsonParseError;
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let content_type = req.headers().get("content-type");
    let body = req.body();
    if content_type.is_none()
      || content_type.is_some_and(|v| v != "application/json")
    {
      Err(JsonParseError::ContentTypeInvalid)
    } else if body.is_empty() {
      Err(JsonParseError::NoBody)
    } else {
      let json = serde_json::from_slice(body).map_err(|inner| JsonParseError::ParseError(inner))?;
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
  fn respond(self) -> Response<String> {
    let body = serde_json::to_string(&self.0).unwrap();
    let mut res = Response::new(body);
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );
    res
  }
}
