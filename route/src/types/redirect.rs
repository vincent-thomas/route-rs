use std::task::Poll;

use route_core::Respondable;
use route_core::Service;
use route_http::{
  body::Body, header, request::Request, response::Response, StatusCode,
};
use route_utils::BoxedFuture;
use route_utils::BoxedSendFuture;

pub struct Redirect {
  to: &'static str,
  permanent: bool,
}

impl Redirect {
  pub fn new(permanent: bool, to: &'static str) -> Redirect {
    Redirect { to, permanent }
  }

  fn gen_response(&self) -> Response {
    let mut res = Response::new(Body::from(()));
    *res.status_mut() = if self.permanent {
      StatusCode::PERMANENT_REDIRECT
    } else {
      StatusCode::TEMPORARY_REDIRECT
    };

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
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, _req: Request) -> Self::Future {
    let response = self.gen_response();
    Box::pin(async move { Ok(response) })
  }
}

impl Respondable for Redirect {
  fn respond(self) -> Response {
    self.gen_response()
  }
}
