use route_core::{FromRequest, Respondable};
use route_http::{request::HttpRequest, response::HttpResponse};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

use super::BodyParseError;

#[derive(Clone)]
pub struct UrlEncoded<T>(pub T);

impl<T> FromRequest for UrlEncoded<T>
where
  T: DeserializeOwned,
{
  type Error = BodyParseError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: HttpRequest) -> Self::Future {
    Box::pin(async move {
      let content_type = req.headers().get("content-type");
      let body = req.body();
      if content_type.is_none()
        || content_type.is_some_and(|v| v != "application/x-www-form-urlencoded")
      {
        Err(BodyParseError::ContentTypeInvalid)
      } else if body.is_empty() {
        Err(BodyParseError::NoBody)
      } else {
        let json = serde_urlencoded::from_bytes(body).unwrap();
        Ok(UrlEncoded(json))
      }
    })
  }
}

// impl<S> Respondable for UrlEncoded<S>
// where
//   S: Serialize,
// {
//   fn respond(self, _req: &HttpRequest) -> HttpResponse {
//     let body = serde_urlencoded::to_string(&self.0).unwrap();
//     let mut res = HttpResponse::new(body);
//     let headers = res.headers_mut();

//     headers.insert(
//       route_http::header::CONTENT_TYPE,
//       route_http::mime::APPLICATION_WWW_FORM_URLENCODED.to_string().parse().unwrap(),
//     );
//     res
//   }
// }

impl<S> Respondable for UrlEncoded<S>
where
  S: Serialize,
{
  fn respond(self) -> HttpResponse {
    let body = serde_urlencoded::to_string(&self.0).unwrap();
    let mut res = HttpResponse::new(body.as_bytes().into());
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_WWW_FORM_URLENCODED.to_string().parse().unwrap(),
    );
    res
  }
}
