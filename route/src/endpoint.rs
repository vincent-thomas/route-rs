use std::collections::HashMap;

use crate::respond::Respondable;
use route_http::{request::Head, response::Response, StatusCode};

pub enum GuardReason {
  Unauthorized,
  Forbidden,
  BadRequest,
  NotFound,
  InternalServerError,
  Custom(String),
}

impl Respondable for GuardReason {
  fn respond(self) -> Response<Box<[u8]>> {
    let status = match self {
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      GuardReason::Custom(_) => StatusCode::BAD_REQUEST,
    };

    Response::builder().status(status).body([].into()).unwrap()
  }
}

pub enum GuardOutcome {
  WeJustPassinBy,
  Reason(GuardReason),
}

pub trait Guard: Sync + Send {
  fn check(&self, head: &Head) -> GuardOutcome;
}

// struct Service<H, Args>
// where
//   H: Handler<Args>,
//   Args: FromRequest,
// {
//   inner: H,
//   phantom: std::marker::PhantomData<Args>,
// }
//
// impl<H, Args> Service<H, Args>
// where
//   Args: FromRequest,
//   H: Handler<Args>,
// {
//   pub fn new(handler: H) -> Self {
//     Self { inner: handler, phantom: std::marker::PhantomData }
//   }
// }
//
// #[async_trait::async_trait]
// impl<Args, H> HttpService for Service<H, Args>
// where
//   Args: FromRequest + Send + Sync,
//   H: Handler<Args> + Send + Sync,
//   H::Output: Respondable,
//   H::Future: Send,
// {
//   async fn call_service(&self, req: HttpRequest) -> HttpResponse {
//     let from_request = match Args::from_request(req) {
//       Ok(args) => args,
//       Err(e) => {
//         let error: Error = e.into();
//         let mut res =
//           route_http::response::HttpResponse::new(error.message.clone());
//         *res.status_mut() = *error.checked_method();
//         return res;
//       }
//     };
//     let output = self.inner.call(from_request).await;
//
//     output.respond()
//   }
// }

/// Represents a web path with a specific HTTP method.
/// Request: A handler that takes a request and returns a response.
/// Guards can be attached to a Route, which are functions that must return true for the route to be matched.
/// Guards are checked in the order they are added.
pub struct Endpoint {
  pub handler: HashMap<route_http::Method, Box<String>>,
  guards: Vec<Box<dyn Guard>>,
}

impl Endpoint {
  pub fn testing(handler: String) -> Self {
    let mut map = HashMap::new();
    map.insert(route_http::Method::GET, Box::new(handler));
    Self { handler: map, guards: Vec::new() }
  }
}
//
//  pub fn guard(mut self, guard: Box<dyn Guard>) -> Self {
//    self.guards.push(guard);
//    self
//  }
//
//  pub fn guards(&self) -> &Vec<Box<dyn Guard>> {
//    &self.guards
//  }
//}
