use super::HttpRequest;

pub struct HttpRequestExt(pub HttpRequest);

impl HttpRequestExt {
  pub fn from_string(s: String) -> Result<http::Request<Box<[u8]>>, ()> {
    from_string(s)
  }
}

fn from_string(req: String) -> Result<HttpRequest, ()> {
  dbg!(req);
  Err(())
}
