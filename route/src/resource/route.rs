use crate::{endpoint::Endpoint, register_method};
use route_core::Respondable;
use route_http::{
  request::{Head, HttpRequest},
  response::HttpResponse,
  Method, StatusCode,
};
use std::collections::HashMap;

use super::{Guard, GuardOutcome, GuardReason};

pub struct Route {
  router: HashMap<route_http::Method, Endpoint>,
  guards: Vec<Box<dyn Guard>>,
  fallback: Endpoint,
}

impl Default for Route {
  fn default() -> Self {
    let default_endpoint = Endpoint::new(|| async {
      let mut res = HttpResponse::new([].into());
      *res.status_mut() = StatusCode::NOT_FOUND;
      res
    });

    Self { router: HashMap::new(), guards: Vec::new(), fallback: default_endpoint }
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

  /// Prevents a request from reaching the handler if any of the guards return a reason.
  fn prevent(endpoint: &Endpoint, head: &Head) -> Option<GuardReason> {
    let guards = endpoint.guards();
    for guard in guards {
      match guard.check(head) {
        GuardOutcome::WeJustPassinBy => continue,
        GuardOutcome::Reason(reason) => return Some(reason),
      }
    }
    None
  }

  pub async fn run(&'static self, req: HttpRequest) -> HttpResponse {
    let (parts, _body) = req.clone().into_parts();

    let Some(endpoint) = self.at(&parts.method) else {
      let handler = &self.fallback.handler;
      return handler.call_service(req).await;
    };

    if let Some(reason) = Route::prevent(endpoint, &parts) {
      return reason.respond();
    };

    let handler = &self.fallback.handler;
    handler.call_service(req).await
  }
}

// pub struct HttpServiceFactory<'a>(pub &'a Route);

// impl<'a> route_core::service::HttpService for HttpServiceFactory<'a> {
//   fn call_service(
//     &'a self,
//     req: route_http::request::HttpRequest,
//   ) -> std::pin::Pin<
//     Box<dyn std::future::Future<Output = route_http::response::HttpResponse> + Send + 'static>,
//   > {
//     Box::pin(self.0.run(req))
//   }
// }
