use http::response::Parts;

pub use http::response::Response;

pub type HttpResponse = Response<Box<[u8]>>;

pub type Head = Parts;

pub struct HttpResponseExt(pub HttpResponse);

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

    let body_str = unsafe { String::from_utf8_unchecked(body.to_vec()) };
    res.push_str(&format!("\r\n{body_str}"));
    res
  }
}
