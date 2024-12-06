use crate::respond::Respondable;
use crate::FromRequest;
use crate::{body::BoxBody, endpoint::Endpoint, register_method};
use route_http::request::Request;
use route_http::{response::Response, Method, StatusCode};
use std::collections::HashMap;

use super::{utils::check_guards, Guard};

pub struct Route {
  router: HashMap<route_http::Method, Endpoint>,
  guards: Vec<Box<dyn Guard>>,
  fallback: Endpoint,
}

impl Default for Route {
  fn default() -> Self {
    let default_endpoint = Endpoint::new(|| async {
      let box_body = BoxBody::new("404 Not found");
      let mut res = Response::new(box_body);
      *res.status_mut() = StatusCode::NOT_FOUND;
      res
    });

    Self {
      router: HashMap::new(),
      guards: Vec::new(),
      fallback: default_endpoint,
    }
  }
}

impl Route {
  pub fn new() -> Self {
    Route::default()
  }

  fn method(&mut self, method: Method, handler: Endpoint) -> &mut Self {
    self.router.insert(method, handler);
    self
  }

  register_method!(GET, get);
  register_method!(POST, post);
  register_method!(PUT, put);
  register_method!(DELETE, delete);
  register_method!(PATCH, patch);

  pub fn guard<G>(mut self, guard: G) -> Self
  where
    G: Guard + 'static,
  {
    self.guards.push(Box::new(guard));
    self
  }

  pub fn at(&self, method: &route_http::Method) -> Option<&Endpoint> {
    self.router.get(method)
  }

  pub async fn run(&self, req: Request) -> Response<Box<[u8]>> {
    let (parts, _) = req.clone().into_parts();

    let endpoint = match self.at(&parts.method) {
      None => return self.fallback.handler.call_service(req).await,
      Some(endpoint) => endpoint,
    };

    if let Some(guard_reason) = check_guards(&self.guards, &parts) {
      return guard_reason.respond();
    }

    endpoint.handler.call_service(req).await
  }
}
