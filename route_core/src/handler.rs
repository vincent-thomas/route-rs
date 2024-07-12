use std::{future::Future, pin::Pin};

use route_http::{request::HttpRequest, response::HttpResponse};

use crate::Respondable;

pub trait Handler: Send + 'static {
  type Future: Future<Output = HttpResponse> + 'static;

  fn call(self, req: HttpRequest) -> Self::Future;
}

// impl<F, Fut, O> Handler<HttpRequest, O> for F
// where
//   F: FnOnce(HttpRequest) -> Fut + Send + 'static + Clone,
//   Fut: Future<Output = O> + Send,
//   O: Respondable + 'static + Send,
// {
// type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
// type Response = HttpResponse;
//   fn call(self, req: HttpRequest) -> impl Future<Output = O> {
//     Box::pin(async move {
//       let req1 = req.clone();
//       let func = self(req1).await;
//       func.respond(&req)
//     })
//   }
// }

// impl<Fut, Fun, Res> Handler<Res> for Fun
// where
//   Fun: FnOnce(HttpRequest) -> Fut + 'static + Send,
//   Fut: Future<Output = Res> + Send,
//   Res: Respondable,

impl<Fut, F> Handler for F
where
  F: FnOnce(HttpRequest) -> Fut + Send + 'static,
  Fut: Future<Output = HttpResponse> + Send,
{
  type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
  fn call(self, req: HttpRequest) -> Self::Future {
    let req1 = req.clone();
    Box::pin(async move {
      let func = self(req1).await;
      func.respond(&req)
    })
  }
}
