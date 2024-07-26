use crate::{resource::Guard, Service, respond::RespondableV2, FromRequest};
use route_core::Handler;

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
  pub handler: Box<dyn Service>,
  guards: Vec<Box<dyn Guard>>,
}

impl Endpoint {
  pub fn new<H, Args>(handler: H) -> Self
  where
    Args: FromRequest + Send + Sync + 'static,
    H: Handler<Args> + Send + Sync,
    H::Output: RespondableV2,
    H::Future: Send,
  {
    let service = Service::new(handler);
    Self { handler: Box::new(service), guards: Vec::new() }
  }

  pub fn guard(mut self, guard: Box<dyn Guard>) -> Self {
    self.guards.push(guard);
    self
  }

  pub fn guards(&self) -> &Vec<Box<dyn Guard>> {
    &self.guards
  }
}
