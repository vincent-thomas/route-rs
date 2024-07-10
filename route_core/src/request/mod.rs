pub mod extractors;
use std::convert::Infallible;

use route_http::request::HttpRequest;

pub trait RequestExtractor<T, R> {
  fn extract(&self) -> Result<T, R>;
}
impl RequestExtractor<HttpRequest, Infallible> for HttpRequest {
  fn extract(&self) -> Result<HttpRequest, Infallible> {
    Ok(self.clone())
  }
}
