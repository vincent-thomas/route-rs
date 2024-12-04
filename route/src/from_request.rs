use route_http::request::Request;

pub trait FromRequest: Sized {
  type Error: Into<()>;
  fn from_request(req: Request) -> Result<Self, Self::Error>;
  fn extract(req: Request) -> Result<Self, Self::Error> {
    Self::from_request(req)
  }
}
