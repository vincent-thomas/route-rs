use crate::handler::Handler;
use crate::{
  http::{FromRequest, Method, Respondable},
  utils::BoxCloneService,
};

use std::future::Future;

#[cfg(feature = "types")]
pub use crate::types::*;

macro_rules! impl_method {
  ($( $method_name:ident $method:ident ),*) => {
            $(
    pub fn $method_name<H, Args>(handler: H) -> $crate::endpoint::Endpoint
    where
      H: Handler<Args> + Sync + Clone,
      H::Future: Future<Output = H::Output> + Send,
      H::Output: Respondable,
      Args: FromRequest + Send + Sync + 'static,
      Args::Error: Send

    {
      let mut methods = std::collections::HashMap::default();
      let route = $crate::route::Route::new(handler);

      methods.insert(Method::$method, BoxCloneService::new(route));
      $crate::endpoint::Endpoint { methods }
    }
            )*
  };
}

pub fn any<H, Args>(handler: H) -> crate::endpoint::Endpoint
where
  H: Handler<Args> + Sync + Clone,
  H::Future: Future<Output = H::Output> + Send,
  H::Output: Respondable,
  Args: FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  let mut methods = std::collections::HashMap::default();
  let route = crate::route::Route::new(handler);

  methods.insert(Method::GET, BoxCloneService::new(route.clone()));
  methods.insert(Method::POST, BoxCloneService::new(route.clone()));
  methods.insert(Method::PUT, BoxCloneService::new(route.clone()));
  methods.insert(Method::DELETE, BoxCloneService::new(route.clone()));
  methods.insert(Method::PATCH, BoxCloneService::new(route));
  crate::endpoint::Endpoint { methods }
}

impl_method!(get GET, post POST, put PUT, delete DELETE, patch PATCH);

//pub fn with_guard<G, T>(guard: G, service: T) -> GuardLayerService<T>
//where
//  G: Guard + 'static,
//  T: Service<Request>,
//{
//  GuardLayerService { guard: Box::new(guard), service }
//}

//pub struct GuardLayerService<S> {
//  guard: Box<dyn Guard>,
//  service: S,
//}

//impl<S> Service<Request> for GuardLayerService<S>
//where
//  S: Service<Request, Response = Response, Error = Response>,
//  S::Future: 'static + Send,
//{
//  type Response = Response;
//  type Error = Response;
//  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;
//  fn poll_ready(
//    &mut self,
//    cx: &mut std::task::Context<'_>,
//  ) -> std::task::Poll<Result<(), Self::Error>> {
//    self.service.poll_ready(cx).map_err(|err| err.respond())
//  }
//
//  fn call(&mut self, req: Request) -> Self::Future {
//    let (parts, body) = req.into_parts();
//    let result = self.guard.check(&parts);
//
//    match result {
//      crate::guard::GuardOutcome::Reason(reason) => {
//        Box::pin(async move { Ok(Response::from(reason)) })
//      }
//      crate::guard::GuardOutcome::WeJustPassinBy => {
//        let req = Request::from_parts(parts, body);
//        Box::pin(self.service.call(req))
//      }
//    }
//  }
//}
