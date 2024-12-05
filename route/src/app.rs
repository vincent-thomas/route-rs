use std::{future::Future, pin::Pin};

use matchit::Router;
use route_http::{
  body::Body,
  request::Request,
  response::{Response, ResponseBuilder},
};
use route_utils::BoxedFuture;
use tower::Service;

#[derive(Default)]
pub struct App {
  pub router: Router<BoxedService<Response>>,
}

pub(crate) type BoxedService<Res> = Box<
  dyn tower::Service<
    Request,
    Response = Res,
    Error = Res,
    Future = Pin<Box<dyn Future<Output = Result<Res, Res>>>>,
  >,
>;

unsafe impl Send for App {}

impl App {
  pub fn at<S>(&mut self, path: &str, endpoint: S) -> &mut Self
  where
    S: Service<
        Request,
        Response = Response,
        Error = Response,
        Future = Pin<Box<dyn Future<Output = Result<Response, Response>>>>,
      > + 'static,
  {
    self.router.insert(path, Box::new(endpoint)).unwrap();
    self
  }
}

impl App {
  pub fn route(&self, path: &str) -> &BoxedService<Response> {
    match self.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("panic"),
    }
  }
  pub fn route_mut<'a>(
    &'a mut self,
    path: &str,
  ) -> &'a mut dyn tower::Service<
    Request,
    Response = Response,
    Error = Response,
    Future = BoxedFuture<Result<Response, Response>>,
  > {
    match self.router.at_mut(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("oh no"),
    }
  }
}

impl tower::Service<Request> for App {
  type Response = Response<Body>;
  type Error = Response<Body>;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Request) -> Self::Future {
    let uri = req.uri().clone();
    let Ok(endpoint) = self.router.at_mut(uri.path()) else {
      let response =
        ResponseBuilder::new().status(404).body(Body::from(())).unwrap();
      return Box::pin(AppFuture { fut: async move { Ok(response) } });
    };

    Box::pin(AppFuture { fut: endpoint.value.call(req) })
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
