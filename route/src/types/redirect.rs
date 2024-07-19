use route_core::Respondable;
use route_http::{header, response::HttpResponse, StatusCode};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::TEMPORARY_REDIRECT }
  }
  fn gen_response(&self) -> HttpResponse {
    let mut res = HttpResponse::new(Box::new([]));
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}

// impl HttpService for Redirect {
//   fn call_service(
//     &self,
//     _req: route_http::request::HttpRequest,
//   ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HttpResponse> + Send + 'static>> {
//     Box::pin(async move { self.gen_response() })
//   }
// }

impl Respondable for Redirect {
  fn respond(self) -> route_http::response::HttpResponse {
    self.gen_response()
  }
}
