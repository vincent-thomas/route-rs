use route_core::Respondable;
use route_http::{
  body::Body, header::HeaderName, request::Request, response::Response,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::FromRequest;

use super::BodyParsingError;

#[derive(Clone)]
pub struct UrlEncoded<T>(pub T);

impl<T> FromRequest for UrlEncoded<T>
where
  T: DeserializeOwned,
{
  type Error = BodyParsingError;
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let content_type = req.headers().get("content-type");
    let body = req.body();
    if content_type.is_none()
      || content_type.is_some_and(|v| v != "application/x-www-form-urlencoded")
    {
      Err(BodyParsingError::ContentTypeInvalid)
    } else if body.is_empty() {
      Err(BodyParsingError::NoBody)
    } else {
      let json = serde_urlencoded::from_bytes(body).unwrap();
      Ok(UrlEncoded(json))
    }
  }
}

impl<W: Serialize> Serialize for UrlEncoded<W> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<S> Respondable for UrlEncoded<S>
where
  S: Serialize,
{
  fn respond(self) -> Response {
    let body = serde_urlencoded::to_string(&self.0).unwrap();
    let body_len = body.len();
    let mut res = Response::new(Body::from(body));
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_WWW_FORM_URLENCODED
        .to_string()
        .parse()
        .unwrap(),
    );

    headers.insert(route_http::header::CONTENT_LENGTH, body_len.into());
    res
  }
}
