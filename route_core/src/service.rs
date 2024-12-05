use std::future::Future;

use route_http::request::Request;

pub trait Service: Send {
  type Response;
  type Future: Future<Output = Self::Response>;
  fn call_service(&self, req: Request) -> Self::Future;
}
