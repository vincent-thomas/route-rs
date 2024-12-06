use std::task::Poll;

use route_core::Respondable;
use route_core::Service;
use route_http::{
  body::Body, header, request::Request, response::Response, StatusCode,
};
use route_utils::BoxedFuture;

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::PERMANENT_REDIRECT }
  }

  fn gen_response(&self) -> Response {
    let mut res = Response::new(Body::from(()));
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}

impl Service<Request> for Redirect {
  type Response = Response;
  type Error = Response;
  type Future = BoxedFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, _req: Request) -> Self::Future {
    let test = self.gen_response();
    Box::pin(async move { Ok(test) })
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
  fn respond(self) -> Response {
    self.gen_response()
  }
}
