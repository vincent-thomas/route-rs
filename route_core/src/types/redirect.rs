use crate::Respondable;
use route_http::{header, response::HttpResponse, status::StatusCode};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::TEMPORARY_REDIRECT }
  }
}

impl Respondable for Redirect {
  fn respond(self, _req: &route_http::request::HttpRequest) -> route_http::response::HttpResponse {
    let mut res = HttpResponse::new("".to_string());
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}
