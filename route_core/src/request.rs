use std::future::Future;

use route_http::request::HttpRequest2;
// pub trait FromRequest: Sized /*+ Clone*/ {
//   type Error;
//   type Future: Future<Output = Result<Self, Self::Error>>;
//   fn from_request(req: &'static HttpRequest) -> Self::Future;
//   fn extract(req: &'static HttpRequest) -> Self::Future {
//     Self::from_request(req)
//   }
// }

pub trait FromRequest2: Sized {
  type Error;
  type Future: Future<Output = Result<Self, Self::Error>> + 'static;
  fn from_request(req: &'static HttpRequest2) -> Self::Future;
  fn extract(req: &'static HttpRequest2) -> Self::Future {
    Self::from_request(req)
  }
}
