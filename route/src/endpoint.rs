use std::{collections::HashMap, future::Future, pin::Pin, task::Poll};

use crate::prelude::*;
use pin_project_lite::pin_project;
use route_core::{FromRequest, Handler, Respondable, Service};
use route_http::{
  body::Body, request::Request, response::Response, Method, StatusCode,
};
use route_utils::BoxedSendFuture;

/// Represents a web path with a specific HTTP method.
///
/// Guards can be attached to a Route, which are functions that must return true for the route to be matched.
/// Guards are checked in the order they are added.
#[derive(Default, Clone)]
pub struct Endpoint {
  pub(crate) methods:
    HashMap<Method, BoxCloneService<Request, Response, Response>>,
}

impl Endpoint {
  pub fn at(
    &self,
    method: &Method,
  ) -> Option<&BoxCloneService<Request, Response, Response>> {
    self.methods.get(method)
  }
  pub fn at_mut(
    &mut self,
    method: &Method,
  ) -> Option<&mut BoxCloneService<Request, Response, Response>> {
    self.methods.get_mut(method)
  }
}

impl Service<Request> for Endpoint {
  type Response = Response;
  type Error = Response;
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }
  fn call(&mut self, req: Request) -> Self::Future {
    let (parts, body) = req.into_parts();

    let Some(route) = self.at_mut(&parts.method) else {
      let mut response = Response::new(Body::from(()));

      *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
      return Box::pin(EndpointFuture { fut: async move { Err(response) } });
    };
    let req = Request::from_parts(parts, body);
    Box::pin(EndpointFuture { fut: route.call(req) })
  }
}

pin_project! {
    struct EndpointFuture<Fut>
    where
      Fut: Future,
    {
      #[pin]
      fut: Fut,
    }
}

impl<Fut> Future for EndpointFuture<Fut>
where
  Fut: Future,
{
  type Output = Fut::Output;
  fn poll(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> Poll<Self::Output> {
    let this = self.project();
    this.fut.poll(cx)
  }
}

macro_rules! impl_methodrouter {
  ($( $method_name:ident $method:ident ),*) => {
        impl Endpoint {
            $(
                pub fn $method_name<H, Args>(mut self, route: H) -> Self
                where
                  H: Handler<Args> + Sync + Clone,
                  H::Future: Future<Output = H::Output> + Send,
                  H::Output: Respondable,
                  Args: FromRequest + Send + Sync + 'static,
                  Args::Error: Send
                {
                    let route = $crate::route::Route::new(route);
                    self.methods.insert(route_http::Method::$method, BoxCloneService::new(route));
                    self
                }
            )*
        }
    };
}

impl_methodrouter!(get GET, post POST, put PUT, delete DELETE, patch PATCH);
