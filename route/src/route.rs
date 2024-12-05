use std::{future::Future, marker::PhantomData, pin::Pin, task::Poll};

use route_core::{FromRequest, Handler, Respondable};
use route_http::{request::Request, response::Response};

use crate::guard::Guard;

type PinBoxedFuture<Out> = Pin<Box<dyn Future<Output = Out> + Send>>;

type BoxedHandler<Req, Res> =
  Box<dyn Handler<Req, Output = Res, Future = PinBoxedFuture<Res>>>;

pub struct Route<T, Args>
where
  T: Handler<Args>,
  Args: FromRequest,
{
  handler: T,
  _a: PhantomData<Args>,
  guards: Vec<Box<dyn Guard>>,
}

impl<T, Args> Route<T, Args>
where
  T: Handler<Args>,
  Args: FromRequest + Send + 'static,
{
  pub fn new(handler: T) -> Self {
    Self { guards: vec![], handler, _a: PhantomData }
  }
}

impl<T, Args> tower::Service<Request> for Route<T, Args>
where
  T: Handler<Args> + Sync + Clone + 'static,
  T::Future: Future<Output = T::Output> + Send + 'static,
  T::Output: Respondable,
  Args: FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  type Response = Response;
  type Error = Response;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Request) -> Self::Future {
    let handler = self.handler.clone();
    Box::pin(async move {
      let response: Result<Args, <Args as FromRequest>::Error> =
        FromRequest::extract(req);
      match response {
        Ok(value) => {
          let result = Handler::call(&handler, value).await;

          Ok(result.respond())
        }
        Err(_err) => Err(_err.respond()),
      }
    })
  }
}
