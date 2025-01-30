use super::{Body, Response};

pub struct HttpResponseExt(pub Response);

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
