use route_http::request::HttpRequest;
use route_http::response::HttpResponse;

use super::Guard;

use super::Route;

#[derive(Default)]
pub struct Resource {
  route: Route,
  guards: Vec<Box<dyn Guard>>,
}

impl Resource {
  pub fn new() -> Self {
    Self { route: Route::default(), guards: vec![] }
  }

  pub fn route(mut self, route: Route) -> Self {
    self.route = route;
    self
  }

  pub fn guard<G>(mut self, guard: G) -> Self
  where
    G: Guard + 'static,
  {
    self.guards.push(Box::new(guard));
    self
  }

  pub fn route_ref(&self) -> &Route {
    &self.route
  }

  pub async fn run(&'static self, request: HttpRequest) -> HttpResponse {
    self.route.run(request).await
  }
}