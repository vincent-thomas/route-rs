
use route_http::{request::Request, response::Response};

use crate::{FromRequest, respond::Respondable};

use super::Infallible;

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

impl FromRequest for Bytes {
  type Error = Infallible;
  fn from_request(
    req: Request,
  ) -> Result<Self, Self::Error> {
    let body = req.body();
    let new_body = body.to_vec();

    Ok(Bytes(new_body))
  }
}
impl Respondable for Bytes {
  fn respond(self) -> Response<Bytes> {
    let inner = self.0;
    Response::new(inner.into())
  }
}
