use route_core::{FromRequest, Respondable};
use route_http::response::HttpResponse;

use super::Infallible;

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

impl FromRequest for Bytes {
  type Error = Infallible;
  fn from_request(
    req: route_http::request::HttpRequest,
  ) -> Result<Self, Self::Error> {
    let body = req.body();
    let new_body = body.to_vec();

    Ok(Bytes(new_body))
  }
}
impl Respondable for Bytes {
  fn respond(self) -> HttpResponse {
    let inner = self.0;
    HttpResponse::new(inner.into())
  }
}
