use std::{collections::HashMap, future::Future, pin::Pin};

use route_core::{service::Service, Handler};
use route_core::{FromRequest, Respondable};
use route_http::{body::Body, response::Response, Method};

use crate::{guard::Guard, route::Route};

type BoxedService<Res> = Box<
  dyn Service<
    Response = Res,
    Future = Pin<Box<dyn Future<Output = Res> + Send>>,
  >,
>;

/// Represents a web path with a specific HTTP method.
/// Request: A handler that takes a request and returns a response.
/// Guards can be attached to a Route, which are functions that must return true for the route to be matched.
/// Guards are checked in the order they are added.
#[derive(Default)]
pub struct Endpoint {
  pub methods: HashMap<Method, BoxedService<Response<Body>>>,
  guards: Vec<Box<dyn Guard>>,
}

impl Endpoint {
  pub fn at(&self, method: &Method) -> Option<&BoxedService<Response<Body>>> {
    self.methods.get(method)
  }
  pub fn at_mut(
    &mut self,
    method: &Method,
  ) -> Option<&mut BoxedService<Response<Body>>> {
    self.methods.get_mut(method)
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
      Args: FromRequest + Send + Sync + 'static
                {
                    let route = Route::new(route);
                    self.methods.insert(route_http::Method::$method, Box::new(route));
                    self
                }
            )*
        }
    };
}

macro_rules! impl_method {
  ($( $method_name:ident $method:ident ),*) => {
            $(
    pub fn $method_name<H, Args>(handler: H) -> Endpoint
    where
      H: Handler<Args> + Sync + Clone,
      H::Future: Future<Output = H::Output> + Send,
      H::Output: Respondable,
      Args: FromRequest + Send + Sync + 'static

    {
      let mut methods = HashMap::default();
      let route = Route::new(handler);
      let boxed: BoxedService<Response<Body>> = Box::new(route);

      methods.insert(route_http::Method::$method, boxed);
      Endpoint { methods, guards: Vec::new() }
    }
            )*
  };
}

impl_method!/*  */(get GET, post POST, put PUT, delete DELETE, patch PATCH);
impl_methodrouter!(get GET, post POST, put PUT, delete DELETE, patch PATCH);
