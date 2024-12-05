use http::{HeaderMap, HeaderValue, StatusCode};

pub type Response<B = Body> = http::Response<B>;

use crate::body::Body;
pub type ResponseBuilder = http::response::Builder;

pub struct Head {
  /// The response's status
  pub status: StatusCode,

  /// The response's headers
  pub headers: HeaderMap<HeaderValue>,
}

impl From<Head> for String {
  fn from(head: Head) -> Self {
    let mut res = format!(
      "HTTP/1.1 {status} {method}\r\n",
      status = head.status.as_u16(),
      method = head.status.as_str()
    );
    for (name, value) in head.headers {
      let header_line =
        format!("{}: {}\r\n", name.unwrap().as_str(), value.to_str().unwrap());
      res.push_str(&header_line);
    }
    res
  }
}

//pub type Head = Parts;

pub struct HttpResponseExt(pub Response<Body>);

impl From<HttpResponseExt> for String {
  fn from(res: HttpResponseExt) -> Self {
    let (parts, body) = res.0.into_parts();
    let status = parts.status;
    let mut res = format!(
      "HTTP/1.1 {status} {method}\r\n",
      status = status.as_u16(),
      method = status.as_str()
    );

    for (name, value) in parts.headers {
      let header_line =
        format!("{}: {}\r\n", name.unwrap().as_str(), value.to_str().unwrap());
      res.push_str(&header_line);
    }

    let body_str: String = body.into();
    res.push_str(&format!("\r\n{body_str}"));
    res
  }
}
