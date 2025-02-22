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

pub struct HttpResponseExt(pub Response<Body>);

impl HttpResponseExt {
  pub fn parse_parts(self) -> (String, Body) {
    let (parts, body) = self.0.into_parts();
    let mut res = format!(
      "HTTP/1.1 {status} {text}\r\n",
      status = parts.status.as_u16(),
      text = parts.status.canonical_reason().unwrap()
    );

    for (name, value) in parts.headers {
      let header_line =
        format!("{}: {}\r\n", name.unwrap().as_str(), value.to_str().unwrap());
      res.push_str(&header_line);
    }
    (res, body)
  }
}

//impl From<HttpResponseExt> for String {
//  fn from(res: HttpResponseExt) -> Self {
//    let (parts, body) = res.0.into_parts();
//    let mut res = format!(
//      "HTTP/1.1 {status} {text}\r\n",
//      status = parts.status.as_u16(),
//      text = parts.status.canonical_reason().unwrap()
//    );
//
//    for (name, value) in parts.headers {
//      let header_line =
//        format!("{}: {}\r\n", name.unwrap().as_str(), value.to_str().unwrap());
//      res.push_str(&header_line);
//    }
//
//    let body_str: String = body.into();
//    res.push_str(&format!("\r\n{body_str}"));
//    res
//  }
//}
