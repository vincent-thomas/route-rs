use route_http::body::Body;
use std::{future::Future, marker::PhantomData, pin::Pin};

use route_core::{service::Service, FromRequest, Handler, Respondable};
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

impl<T, Args> Service for Route<T, Args>
where
  T: Handler<Args> + Sync + Clone + 'static,
  T::Future: Future<Output = T::Output> + Send + 'static,
  T::Output: Respondable,
  Args: FromRequest + Send + Sync + 'static,
{
  type Response = Response<Body>;
  type Future = Pin<Box<dyn Future<Output = Self::Response> + Send>>;
  fn call_service(&self, req: Request) -> Self::Future {
    let handler = self.handler.clone();
    Box::pin(async move {
      let result = match FromRequest::extract(req) {
        Ok(value) => value,
        Err(_err) => panic!("erro"),
      };
      let result = handler.call(result).await;

      result.respond()
    })
  }
}

//impl<T, Args> Service for HandlerService<T, Args>
//where
//  T: Handler<Args> + Sync,
//  Args: FromRequest + Send + Sync,
//  T::Future: Future<Output = T::Output> + Send,
//  T::Output: Respondable,
//{
//  type Future = Pin<Box<dyn Future<Output = Self::Response> + Send>>;
//  type Response = Response<Body>;
//
//  fn call_service(&'static self, req: Request) -> Self::Future {
//    Box::pin(async move {
//      let result = match crate::FromRequest::extract(req) {
//        Ok(value) => value,
//        Err(_err) => panic!("erro"),
//      };
//      let result = self.handler.call(result).await;
//
//      result.respond()
//    })
//  }
//}
