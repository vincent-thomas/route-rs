use async_trait::async_trait;
use route_http::{request::HttpRequest2, response::HttpResponse2};
use std::{fmt::Debug, future::Future};

use crate::Respondable2;

// pub trait Handler: Send + 'static {
//   type Future: Future<Output = HttpResponse> + 'static;

//   fn call(self, req: HttpRequest) -> Self::Future;
// }

pub trait Handler2: Send + Sync + 'static {
  type Future: Future<Output = HttpResponse2> + 'static;

  fn call(self, req: HttpRequest2) -> Self::Future;
}

#[async_trait]
pub trait Endpoint: Send + Sync + 'static {
  async fn call(&self, req: HttpRequest2) -> HttpResponse2;
}

// impl<Fut, F> Handler for F
// where
//   F: FnOnce(HttpRequest) -> Fut + Send + 'static,
//   Fut: Future<Output = HttpResponse> + Send,
// {
//   type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
//   fn call(self, req: HttpRequest) -> Self::Future {
//     let fn_req = req.clone();
//     Box::pin(async move {
//       let func = self(fn_req).await;
//       func.respond(&req)
//     })
//   }
// }

// impl<Fut, F, O> Handler2 for F
// where
//   F: FnOnce(HttpRequest2) -> Fut + Send + 'static,
//   Fut: Future<Output = O> + Send,
//   O: Respondable2,
// {
//   type Future = Pin<Box<dyn Future<Output = HttpResponse2> + Send>>;
//   fn call(self, req: HttpRequest2) -> Self::Future {
//     let fn_req = req.clone();
//     Box::pin(async move {
//       let func = self(fn_req).await;
//       func.respond(&req)
//     })
//   }
// }

#[async_trait]
impl<F, Fut, O> Endpoint for F
where
  F: Send + Sync + 'static + Fn(HttpRequest2) -> Fut,
  Fut: Future<Output = O> + Send + 'static,
  O: Respondable2 + 'static,
{
  async fn call(&self, req: HttpRequest2) -> HttpResponse2 {
    let func = (self)(req).await;
    func.respond()
  }
}

pub type BoxedEndpoint = Box<dyn Endpoint>;
