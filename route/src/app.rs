use std::sync::Arc;

use matchit::Router;
use route_http::{response::Response, request::Request};
//use route_core::service::{HttpService, Service};
//use route_http::response::HttpResponse;

use crate::{panic_err, resource::Resource, Service};

#[derive(Default, Clone)]
pub struct App {
  inner: Arc<AppInner>,
}

struct AppInner {
  pub router: Router<Box<dyn Service>>,
  pub default: Box<dyn Service>,
}

impl Default for AppInner {
  fn default() -> Self {
    AppInner { router: Router::new(), default: Box::new(Resource::new()) }
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

  pub fn at<T>(&mut self, path: &str, service: T) -> &mut Self
  where
    T: Service + 'static,
  {
    self.tap_inner(|inner| {
      let insertion = inner.router.insert(path, Box::new(service));
      panic_err!(insertion, "Failed to insert route into router: {:#?}");
    });
    self
  }
}

#[async_trait::async_trait]
impl Service for Resource {
  async fn call_service(
    &self,
    req: Request,
  ) -> Response<Box<[u8]>> {
    let fut = self.run(req);
    fut.await
  }
}

impl App {
  pub fn route<'a>(&'a self, path: &str) -> &Box<dyn Service + 'a> {
    match self.inner.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => &self.inner.default,
    }
  }

  pub fn route_mut<'a>(&'a mut self, path: &str) -> &'a mut dyn Service {
    self.tap_inner(move |inner| match inner.router.at_mut(path) {
      Ok(thing) => thing.value.as_mut(),
      Err(_) => inner.default.as_mut(),
    })
  }
}
