use std::future::Future;

use route_http::request::HttpRequest;

pub trait FromRequest: Sized {
  type Error;
  type Future: Future<Output = Result<Self, Self::Error>> + 'static;
  fn from_request(req: &'static HttpRequest) -> Self::Future;
  fn extract(req: &'static HttpRequest) -> Self::Future {
    Self::from_request(req)
  }
}
