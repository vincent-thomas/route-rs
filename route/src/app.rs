use std::sync::Arc;

use matchit::Router;
use route_http::response::Response;

use crate::endpoint::Endpoint;
//use route_core::service::{HttpService, Service};
//use route_http::response::HttpResponse;

//use crate::{endpoint::Endpoint, panic_err, resource::Resource, Service};

#[derive(Default, Clone)]
pub struct App {
  inner: Arc<AppInner>,
}

struct AppInner {
  pub router: Router<Endpoint>,
  //pub default: Box<dyn Service>,
}

impl Default for AppInner {
  fn default() -> Self {
    AppInner { router: Router::new() }
  }
}

impl App {
  pub fn new() -> Self {
    App { inner: Arc::new(AppInner::default()) }
  }

  fn tap_inner<'a, F, V>(&'a mut self, f: F) -> V
  where
    F: FnOnce(&'a mut AppInner) -> V,
  {
    let inner = Arc::get_mut(&mut self.inner).unwrap();
    f(inner)
  }

  pub fn at(&mut self, path: &str, endpoint: Endpoint) -> &mut Self {
    self.tap_inner(|inner| inner.router.insert(path, endpoint).unwrap());
    self
  }
}

impl App {
  pub fn route(&self, path: &str) -> &Endpoint {
    match self.inner.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("panic"),
    }
  }
  pub fn route_mut<'a>(&'a mut self, path: &str) -> &'a mut Endpoint {
    self.tap_inner(move |inner| match inner.router.at_mut(path) {
      Ok(thing) => thing.value,
      Err(_) => panic!("oh no"),
    })
  }
}

#[async_trait::async_trait]
impl route_core::service::Service for App {
  async fn call_service(
    &self,
    req: route_http::request::Request,
  ) -> route_http::response::Response<Box<[u8]>> {
    let req_path = req.uri().path();
    let method = req.method();
    let matchit::Match { value: endpoint, params: _ } =
      self.inner.router.at(req_path).unwrap();

    if let Some(result) = endpoint.handler.get(method) {
      let response = Response::new(result.as_bytes().into());

      response
    } else {
      let response = Response::new("No Body :(".as_bytes().into());
      response
    }
  }
}
