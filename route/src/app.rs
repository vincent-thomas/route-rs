use matchit::Router;
use route_core::service::HttpService;
use route_server::findable::FindableRoute;

use crate::{
  panic_err,
  resource::{HttpServiceFactory, Resource},
};

#[derive(Default)]
pub struct App {
  router: Router<Resource>,
  default: Resource,
}

impl App {
  pub fn new() -> Self {
    App { router: Router::default(), default: Resource::default() }
  }

  pub fn at(&mut self, path: &str, service: Resource) -> &mut Self {
    let insertion = self.router.insert(path, service);
    panic_err!(insertion, "Failed to insert route into router: {:#?}");
    self
  }
}

impl FindableRoute<'static> for App {
  fn find_route(&'static self, path: &str) -> Box<dyn HttpService> {
    let matched = match self.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => &self.default,
    };

    Box::new(HttpServiceFactory(matched.route_ref()))
  }
}
