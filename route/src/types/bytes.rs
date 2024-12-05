use route_http::{body::Body, request::Request, response::Response};

use crate::{FromRequest, Respondable};

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

impl FromRequest for Bytes {
  type Error = ();
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let body = req.body();
    let new_body = body.to_vec();

    Ok(Bytes(new_body))
  }
}
impl Respondable for Bytes {
  fn respond(self) -> Response {
    let inner = self.0;
    Response::new(Body::from(inner))
  }
}
