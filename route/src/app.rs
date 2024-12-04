use std::{future::Future, pin::Pin, sync::Arc};

use crate::endpoint::Endpoint;
use matchit::Router;
use route_core::service::Service;
use route_http::{body::Body, response::Response};

#[derive(Default, Clone)]
pub struct App {
  pub service: Arc<AppService>,
}

#[derive(Default)]
pub struct AppService {
  pub router: Router<Endpoint>,
}

unsafe impl Send for AppService {}
unsafe impl Sync for AppService {}

impl App {
  fn tap_inner<'a, F, V>(&'a mut self, f: F) -> V
  where
    F: FnOnce(&'a mut AppService) -> V,
  {
    let inner = Arc::get_mut(&mut self.service).unwrap();
    f(inner)
  }
  pub fn at(&mut self, path: &str, endpoint: Endpoint) -> &mut Self {
    self.tap_inner(|app| app.router.insert(path, endpoint).unwrap());
    self
  }
}

impl App {
  pub fn route(&self, path: &str) -> &Endpoint {
    match self.service.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("panic"),
    }
  }
  pub fn route_mut<'a>(&'a mut self, path: &str) -> &'a mut Endpoint {
    self.tap_inner(|app| match app.router.at_mut(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("oh no"),
    })
  }
}

impl Service for AppService {
  type Future = Pin<Box<dyn Future<Output = Self::Response> + Send + 'static>>;
  type Response = Response<Body>;
  fn call_service(&self, req: route_http::request::Request) -> Self::Future {
    let req_path = req.uri().path();
    let method = req.method();
    let matchit::Match { value: endpoint, params: _ } =
      self.router.at(req_path).unwrap();

    let Some(result) = endpoint.at(method) else {
      println!("noooo");
      return Box::pin(async { Response::new(Body) });
    };
    result.call_service(req)
  }
}
