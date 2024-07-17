use route_core::{FromRequest, Respondable};
use route_http::{request::HttpRequest, response::HttpResponse};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

use crate::types::BodyParseError;

pub struct Json<T>(pub T);

impl<T> FromRequest for Json<T>
where
  T: DeserializeOwned,
{
  type Error = BodyParseError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: HttpRequest) -> Self::Future {
    Box::pin(async move {
      let content_type = req.headers().get("content-type");
      let body = req.body();
      if content_type.is_none() || content_type.is_some_and(|v| v != "application/json") {
        Err(BodyParseError::ContentTypeInvalid)
      } else if body.is_empty() {
        Err(BodyParseError::NoBody)
      } else {
        let json = serde_json::from_slice(body).unwrap();
        Ok(Json(json))
      }
    })
  }
}

impl<S: Serialize> Respondable for Json<S> {
  fn respond(self) -> HttpResponse {
    let body = serde_json::to_string(&self.0).unwrap();
    let mut res = HttpResponse::new(body.as_bytes().into());
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );
    res
  }
}
