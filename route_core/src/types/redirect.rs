use crate::{handler::Handler, Respondable};
use route_http::{header, response::HttpResponse, status::StatusCode};
use std::{future::Future, pin::Pin};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::TEMPORARY_REDIRECT }
  }
}

impl Handler for Redirect {
  type Future = Pin<Box<dyn Future<Output = HttpResponse>>>;
  fn call(self, _req: route_http::request::HttpRequest) -> Self::Future {
    Box::pin(async { gen_response(self) })
  }
}

impl Respondable for Redirect {
  fn respond(self, _req: &route_http::request::HttpRequest) -> route_http::response::HttpResponse {
    gen_response(self)
  }
}

fn gen_response(redirect: Redirect) -> HttpResponse {
  let mut res = HttpResponse::new("".to_string());
  *res.status_mut() = redirect.status_code;

  let headers = res.headers_mut();

  if let Ok(location) = redirect.to.parse() {
    headers.insert(header::LOCATION, location);
  }

  res
}
