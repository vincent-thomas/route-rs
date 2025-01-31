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

#[cfg(test)]
mod tests {
  use http::StatusCode;

  use crate::http::{
    response::Builder, response_parser::HttpResponseExt, Body,
  };

  #[test]
  fn test_http_response_ext_parse_parts() {
    let response = Builder::default()
      .status(StatusCode::OK)
      .header("Content-Type", "text/plain")
      .body(Body::from("Hello, world!"))
      .unwrap();

    let http_ext = HttpResponseExt(response);
    let (parsed_response, body) = http_ext.parse_parts();

    assert!(parsed_response.contains("HTTP/1.1 200 OK"));
    assert!(parsed_response.contains("content-type: text/plain"));

    match body {
      Body::Full(full) => {
        let to_compare: Box<[u8]> =
          String::from("Hello, world!").as_bytes().into();

        assert!(full == to_compare)
      }
      Body::Stream(_) => unreachable!(),
    }
  }
}
