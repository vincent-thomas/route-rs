use std::task::Poll;

use titan_core::Respondable;
use titan_core::Service;
use titan_http::{body::Body, header, Request, Response, StatusCode};
use titan_utils::BoxedSendFuture;

#[derive(Clone)]
pub struct Redirect {
  to: &'static str,
  permanent: bool,
}

impl Redirect {
  pub fn new(permanent: bool, to: &'static str) -> Redirect {
    Redirect { to, permanent }
  }

  pub fn permanent(to: &'static str) -> Redirect {
    Redirect { permanent: true, to }
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
