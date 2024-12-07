use crate::prelude::*;
use route_core::Service;
use route_http::{
  body::Body,
  request::Request,
  response::{Response, ResponseBuilder},
};
use route_router::Router;
use route_utils::BoxedSendFuture;
use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin};

#[derive(Default)]
pub struct App {
  router: Router<BoxedSendService<Response>>,
}

impl App {
  pub fn at<S>(&mut self, path: &str, endpoint: S) -> &mut Self
  where
    S: Service<
        Request,
        Response = Response,
        Error = Response,
        Future = BoxedSendFuture<Result<Response, Response>>,
      > + 'static,
  {
    self.router.at(path, Box::new(endpoint));
    self
  }
}

impl Service<Request> for App {
  type Response = Response<Body>;
  type Error = Response<Body>;
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, mut req: Request) -> Self::Future {
    let uri = req.uri().clone();
    match self.router.at_mut(uri.path()) {
      Some(endpoint) => {
        let params: HashMap<String, Value> =
          HashMap::from_iter(endpoint.params.iter().map(|(key, value)| {
            (key.to_string(), Value::from(value.to_string()))
          }));
        let mut extensions = route_http::Extensions::new();
        extensions.insert(params);

        *req.extensions_mut() = extensions;

        Box::pin(AppFuture { fut: endpoint.value.call(req) })
      }
      None => {
        let response =
          ResponseBuilder::new().status(404).body(Body::from(())).unwrap();
        Box::pin(AppFuture { fut: async move { Ok(response) } })
      }
    }
  }
}

pin_project_lite::pin_project! {
  struct AppFuture<F> {
      #[pin]
      fut: F
  }
}

impl<F> Future for AppFuture<F>
where
  F: Future,
{
  type Output = F::Output;

  fn poll(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let this = self.project();

    this.fut.poll(cx)
  }
}
