use route_core::{FromRequest, Respondable};
use route_http::response::HttpResponse;

use std::future::Future;
use std::pin::Pin;

use super::Infallible;

pub struct Bytes<T>(pub T);

impl FromRequest for Bytes<Box<[u8]>> {
  type Error = Infallible;
  fn from_request(req: route_http::request::HttpRequest) -> Result<Self, Self::Error> {
    let body = req.body().clone();
    Ok(Bytes(body))
  }
}

impl FromRequest for Bytes<Vec<u8>> {
  type Error = Infallible;
  fn from_request(req: route_http::request::HttpRequest) -> Result<Self, Self::Error> {
    let body = req.body();
    let new_body = body.to_vec();

    Ok(Bytes(new_body))
  }
}

impl Respondable for Bytes<Box<[u8]>> {
  fn respond(self) -> HttpResponse {
    let inner = self.0;
    HttpResponse::new(inner)
  }
}

impl<'a> Respondable for Bytes<&'a [u8]> {
  fn respond(self) -> HttpResponse {
    let inner = self.0.into();
    HttpResponse::new(inner)
  }
}

impl Respondable for Bytes<Vec<u8>> {
  fn respond(self) -> HttpResponse {
    let inner = self.0;
    HttpResponse::new(inner.into())
  }
}

impl<'a> Respondable for Bytes<&'a str> {
  fn respond(self) -> HttpResponse {
    let inner = self.0;
    let body = inner.as_bytes().into();
    HttpResponse::new(body)
  }
}