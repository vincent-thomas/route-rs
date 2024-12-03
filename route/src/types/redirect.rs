use async_trait::async_trait;
use route_http::{header, request::Request, response::Response, StatusCode};

use crate::{respond::Respondable, Service};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::PERMANENT_REDIRECT }
  }
  fn gen_response(&self) -> Response<Box<[u8]>> {
    let mut res = Response::new(Box::new([]));
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}

#[async_trait]
impl Service for Redirect {
  async fn call_service(&self, _req: Request) -> Response<Box<[u8]>> {
    let test = self.gen_response();
    test
    //let res = Response::builder().header("Location", );
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
  fn respond(self) -> Response<Box<[u8]>> {
    self.gen_response()
  }
}
