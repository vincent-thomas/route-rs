use route_http::body::Body;
use std::{
  future::Future,
  marker::PhantomData,
  pin::{pin, Pin},
};

use route_http::{request::Request, response::Response};

use crate::{FromRequest, Handler, Respondable};

pub trait Service: Send {
  type Response;
  type Future: Future<Output = Self::Response>;
  fn call_service(&self, req: Request) -> Self::Future;
}

pub struct HandlerService<H, Args>
where
  H: Handler<Args>,
{
  handler: H,
  _a: PhantomData<Args>,
}

impl<H, Args> HandlerService<H, Args>
where
  H: Handler<Args>,
{
  pub fn new(handler: H) -> Self
  where
    Args: FromRequest,
  {
    HandlerService { handler, _a: PhantomData }
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
//  fn call_service(&self, req: Request) -> Self::Future {
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
