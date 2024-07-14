use route_core::{FromRequest, Respondable};
use route_http::response::HttpResponse;

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;

pub struct Bytes<T>(pub T);

impl FromRequest for Bytes<Box<[u8]>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequest) -> Self::Future {
    let body = req.body();
    Box::pin(async { Ok(Bytes(body.clone())) })
  }
}

impl FromRequest for Bytes<Vec<u8>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequest) -> Self::Future {
    let body = req.body();
    let new_body = body.to_vec();

    Box::pin(async { Ok(Bytes(new_body)) })
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
