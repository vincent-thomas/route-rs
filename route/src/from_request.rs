use route_http::request::Request;

use crate::error::Error;

pub trait FromRequest: Sized {
  type Error: Into<Error>;
  fn from_request(req: Request) -> Result<Self, Self::Error>;
  fn extract(req: Request) -> Result<Self, Self::Error> {
    Self::from_request(req)
  }
}
