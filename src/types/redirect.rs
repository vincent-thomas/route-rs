use crate::{handler::Handler2, Respondable2};
use route_http::{header, response::HttpResponse2, status::StatusCode};
use std::{future::Future, pin::Pin};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::TEMPORARY_REDIRECT }
  }
  fn gen_response(self: Redirect) -> HttpResponse2 {
    let mut res = HttpResponse2::new(Box::new([]));
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}

impl Handler2 for Redirect {
  type Future = Pin<Box<dyn Future<Output = HttpResponse2>>>;
  fn call(self, _req: route_http::request::HttpRequest2) -> Self::Future {
    Box::pin(async { self.gen_response() })
  }
}

impl Respondable2 for Redirect {
  fn respond(self) -> route_http::response::HttpResponse2 {
    self.gen_response()
  }
}
