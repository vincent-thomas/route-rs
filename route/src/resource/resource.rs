use route_http::request::Request;
use route_http::response::Response;

use crate::respond::Respondable;

use super::utils::check_guards;
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

  pub async fn run(&self, request: Request) -> Response<Box<[u8]>> {
    let (parts, _) = request.clone().into_parts();
    match check_guards(&self.guards, &parts) {
      Some(reason) => Respondable::respond(reason),
      None => self.route.run(request).await.respond(),
    }
  }

  // pub async fn run2(&self, request: Request) -> Response {
  //   let (parts, _) = request.clone().into_parts();
  //   match check_guards(&self.guards, &parts) {
  //     Some(reason) => RespondableV2::respond(reason),
  //     None => self.route.run(request).await.respond(),
  //   }
  // }
}
