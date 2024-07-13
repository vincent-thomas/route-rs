use route_core::{FromRequest2, Respondable2};
use route_http::{request::HttpRequest2, response::HttpResponse2};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

use crate::types::BodyParseError;

pub struct Json<T>(pub T);

impl<T> FromRequest2 for Json<T>
where
  T: DeserializeOwned + 'static,
{
  type Error = BodyParseError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &HttpRequest2) -> Self::Future {
    let content_type = req.headers().get("content-type");
    let output = {
      let body = req.body();
      if content_type.is_none() || content_type.is_some_and(|v| v != "application/json") {
        Err(BodyParseError::ContentTypeInvalid)
      } else if body.is_empty() {
        Err(BodyParseError::NoBody)
      } else {
        let json = serde_json::from_slice(body).unwrap();
        Ok(Json(json))
      }
    };

    Box::pin(async move { output })
  }
}

impl<S: Serialize> Respondable2 for Json<S> {
  fn respond(self) -> HttpResponse2 {
    let body = serde_json::to_string(&self.0).unwrap();
    let mut res = HttpResponse2::new(body.as_bytes().into());
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );
    res
  }
}
