use std::collections::HashMap;

use route_core::{BoxedEndpoint, Endpoint};
use route_http::method::Method;

#[derive(Default)]
pub struct EndpointRouter {
  router: HashMap<Method, BoxedEndpoint>,
}

impl EndpointRouter {
  pub fn method(&mut self, method: Method, handler: impl Endpoint) -> &mut Self {
    self.router.insert(method, Box::new(handler));
    self
  }

  pub fn at(&self, method: &Method) -> &BoxedEndpoint {
    let thing = self.router.get(method);

    match thing {
      Some(endpoint) => endpoint,
      None => {
        unimplemented!()
      }
    }
  }
}
