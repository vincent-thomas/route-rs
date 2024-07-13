use route_core::{FromRequest2, Respondable2};
use route_http::response::HttpResponse2;

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;

pub struct Bytes<T>(pub T);

impl FromRequest2 for Bytes<Box<[u8]>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequest2) -> Self::Future {
    let body = req.body();
    Box::pin(async { Ok(Bytes(body.clone())) })
  }
}

impl FromRequest2 for Bytes<Vec<u8>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequest2) -> Self::Future {
    let body = req.body();
    let new_body = body.to_vec();

    Box::pin(async { Ok(Bytes(new_body)) })
  }
}

impl Respondable2 for Bytes<Box<[u8]>> {
  fn respond(self) -> HttpResponse2 {
    let inner = self.0;
    HttpResponse2::new(inner)
  }
}

impl<'a> Respondable2 for Bytes<&'a [u8]> {
  fn respond(self) -> HttpResponse2 {
    let inner = self.0.into();
    HttpResponse2::new(inner)
  }
}

impl Respondable2 for Bytes<Vec<u8>> {
  fn respond(self) -> HttpResponse2 {
    let inner = self.0;
    HttpResponse2::new(inner.into())
  }
}

impl<'a> Respondable2 for Bytes<&'a str> {
  fn respond(self) -> HttpResponse2 {
    let inner = self.0;
    let body = inner.as_bytes().into();
    HttpResponse2::new(body)
  }
}
