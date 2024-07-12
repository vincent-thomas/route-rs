use crate::request::FromRequestV2;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;

pub struct Bytes<T>(T);

impl FromRequestV2 for Bytes<Box<[u8]>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequestV2) -> Self::Future {
    Box::pin(async { Ok(Bytes(req.body().clone())) })
  }
}

impl FromRequestV2 for Bytes<Vec<u8>> {
  type Error = Infallible;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: &'static route_http::request::HttpRequestV2) -> Self::Future {
    let body = req.body();
    let new_body = body.to_vec();

    Box::pin(async { Ok(Bytes(new_body)) })
  }
}
