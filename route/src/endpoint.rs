use std::task::Poll;
use std::{collections::HashMap, future::Future, pin::Pin};

use route_core::Handler;
use route_core::{FromRequest, Respondable};
use route_http::request::Request;
use route_http::StatusCode;
use route_http::{body::Body, response::Response, Method};
use route_utils::BoxedFuture;

use crate::{guard::Guard, route::Route};

pub(crate) type BoxedService<Res> = Box<
  dyn tower::Service<
    Request,
    Response = Res,
    Error = Res,
    Future = Pin<Box<dyn Future<Output = Result<Res, Res>> + Send>>,
  >,
>;

/// Represents a web path with a specific HTTP method.
/// Request: A handler that takes a request and returns a response.
/// Guards can be attached to a Route, which are functions that must return true for the route to be matched.
/// Guards are checked in the order they are added.
#[derive(Default)]
pub struct Endpoint {
  pub(crate) methods: HashMap<Method, BoxedService<Response>>,
  pub(crate) guards: Vec<Box<dyn Guard>>,
}

impl Endpoint {
  pub fn at(&self, method: &Method) -> Option<&BoxedService<Response>> {
    self.methods.get(method)
  }
  pub fn at_mut(
    &mut self,
    method: &Method,
  ) -> Option<&mut BoxedService<Response<Body>>> {
    self.methods.get_mut(method)
  }
}

impl tower::Service<Request> for Endpoint {
  type Future = BoxedFuture<Result<Self::Response, Self::Error>>;
  type Response = Response;
  type Error = Response;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }
  fn call(&mut self, req: Request) -> Self::Future {
    let (parts, _body) = req.clone().into_parts();

    let Some(route) = self.at_mut(&parts.method) else {
      let mut response = Response::new(Body::from(()));

      *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
      return Box::pin(EndpointFuture { fut: async move { Err(response) } });
    };
    Box::pin(EndpointFuture { fut: route.call(req) })
  }
}

use pin_project_lite::pin_project;

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
                    let route = Route::new(route);
                    self.methods.insert(route_http::Method::$method, Box::new(route));
                    self
                }
            )*
        }
    };
}

impl_methodrouter!(get GET, post POST, put PUT, delete DELETE, patch PATCH);
