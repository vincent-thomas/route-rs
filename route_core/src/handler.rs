use std::{future::Future, pin::Pin};

use route_http::{request::HttpRequest, response::HttpResponse};

use crate::Respondable;

pub trait Handler: Send + 'static {
  type Future: Future<Output = HttpResponse> + 'static;

  fn call(self, req: HttpRequest) -> Self::Future;
}

impl<Fut, F> Handler for F
where
  F: FnOnce(HttpRequest) -> Fut + Send + 'static,
  Fut: Future<Output = HttpResponse> + Send,
{
  type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
  fn call(self, req: HttpRequest) -> Self::Future {
    let fn_req = req.clone();
    Box::pin(async move {
      let func = self(fn_req).await;
      func.respond(&req)
    })
  }
}
