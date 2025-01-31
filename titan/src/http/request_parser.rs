use std::str::FromStr as _;

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
  let formated = iter.filter_map(|line| {
    let mut parts: Vec<&str> = line.split(": ").collect();
    let key = parts.remove(0);

    match key.parse() {
      Ok(key) => Some((key, parts[0].to_string().parse().unwrap())),
      Err(_err) => None,
    }
  });

  HeaderMap::from_iter(formated)
}

#[derive(Debug)]
pub enum RequestParsingError {
  HttpProto(Error),
  CompletelyInvalid,
}

fn from_string(str_request: String) -> Result<Request, RequestParsingError> {
  dbg!(&str_request);

  let mut http_req: Vec<String> =
    str_request.split('\n').map(|v| v.to_string()).collect();

  let http_meta = http_req.swap_remove(0);
  let parts: Vec<&str> = http_meta.split_whitespace().collect();

  let method = Method::from_str(
    parts.get(0).ok_or(RequestParsingError::CompletelyInvalid)?,
  )
  .map_err(|err| RequestParsingError::HttpProto(Error::from(err)))?;
  let uri =
    Uri::from_str(parts.get(1).ok_or(RequestParsingError::CompletelyInvalid)?)
      .map_err(|err| RequestParsingError::HttpProto(Error::from(err)))?;

  let headers = format_headers(http_req.into_iter());

  let mut req_builder = http::Request::builder().uri(uri).method(method);

  *req_builder.headers_mut().unwrap() = headers;

  req_builder.body([].into()).map_err(RequestParsingError::HttpProto)
}

#[cfg(test)]
mod tests {
  use super::*;
  use http::{Method, Uri};

  #[test]
  fn test_from_nvalstring_valid_request() {
    let request_str =
      "GET /test HTTP/1.1\nHost: example.com\nUser-Agent: test-agent"
        .to_string();
    let request = from_string(request_str).unwrap();

    assert_eq!(request.method(), &Method::GET);
    assert_eq!(request.uri(), &Uri::from_str("/test").unwrap());
    assert_eq!(request.headers()["Host"], "example.com");
    assert_eq!(request.headers()["User-Agent"], "test-agent");
  }

  #[test]
  fn test_from_string_invalid_request() {
    let request_str = "INVALID_REQUEST".to_string();
    let result = from_string(request_str);
    assert!(result.is_err());
  }

  #[test]
  fn test_format_headers() {
    let headers = vec![
      "Host: example.com".to_string(),
      "User-Agent: test-agent".to_string(),
    ];
    let header_map = format_headers(headers.into_iter());

    assert_eq!(header_map["Host"], "example.com");
    assert_eq!(header_map["User-Agent"], "test-agent");
  }

  #[test]
  fn test_http_request_ext_from_string() {
    let request_str = "GET / HTTP/1.1\nHost: localhost".to_string();
    let http_ext = HttpRequestExt::from(request_str);

    assert_eq!(http_ext.0.method(), &Method::GET);
    assert_eq!(http_ext.0.uri(), &Uri::from_str("/").unwrap());
    assert_eq!(http_ext.0.headers()["Host"], "localhost");
  }
}
