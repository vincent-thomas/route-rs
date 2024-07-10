use std::{future::Future, pin::Pin};

use route_core::request::RequestExtractor;
use route_http::{
  request::HttpRequest,
  response::{HttpResponse, Respondable},
};
pub trait Handler: Send + Sized + 'static {
  type Future: Future<Output = HttpResponse> + Send + 'static;

  fn call(self, req: HttpRequest) -> Self::Future;
}

impl<F, Fut, O> Handler for F
where
  F: FnOnce(HttpRequest) -> Fut + Send + 'static,
  Fut: Future<Output = O> + Send,
  O: Respondable + 'static + Send,
{
  type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
  fn call(self, req: HttpRequest) -> Self::Future {
    Box::pin(async move {
      let func = self(req.extract().unwrap()).await;
      func.respond(&req)
    })
  }
}
