use std::future::Future;

use route_http::request::HttpRequest;
pub trait FromRequest: Sized + Clone {
  type Error;
  type Future: Future<Output = Result<Self, Self::Error>>;
  fn from_request(req: &'static HttpRequest) -> Self::Future;
  fn extract(req: &'static HttpRequest) -> Self::Future {
    Self::from_request(req)
  }
}
