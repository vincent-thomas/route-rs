use std::str::FromStr;

use super::Request;
use http::{Error, HeaderMap, HeaderValue, Method, Uri};

pub struct HttpRequestExt(pub Request);

impl From<String> for HttpRequestExt {
  fn from(value: String) -> Self {
    HttpRequestExt(from_string(value).unwrap())
  }
}

fn format_headers(
  iter: impl Iterator<Item = String>,
) -> HeaderMap<HeaderValue> {
  let formated = iter.map(|line| {
    let mut parts: Vec<&str> = line.split(": ").collect();
    let key = parts.remove(0);
    (key.parse().unwrap(), parts[0].to_string().parse().unwrap())
  });

  HeaderMap::from_iter(formated)
}

fn from_string(str_request: String) -> Result<Request, Error> {
  let mut http_req: Vec<String> =
    str_request.split('\n').map(|v| v.to_string()).collect();

  let http_meta = http_req.swap_remove(0);
  let parts: Vec<&str> = http_meta.split_whitespace().collect();

  dbg!(&parts);

  let method = Method::from_str(parts[0]).unwrap();
  let uri = Uri::from_str(parts[1]).unwrap();

  let headers = format_headers(http_req.into_iter());

  let mut req_builder = http::Request::builder().uri(uri).method(method);

  *req_builder.headers_mut().unwrap() = headers;

  req_builder.body([].into())
}
