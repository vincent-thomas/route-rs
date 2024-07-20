use std::sync::Arc;

use matchit::Router;
use route_core::service::HttpService;
use route_http::{request::HttpRequest, response::HttpResponse};

use crate::{panic_err, resource::Resource};

#[derive(Default)]
pub struct App {
  inner: Arc<AppInner>,
}

struct AppInner {
  pub router: Router<Box<dyn HttpService>>,
  pub default: Box<dyn HttpService>,
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

  fn tap_inner<F>(&mut self, f: F) -> &mut Self
  where
    F: FnOnce(&mut AppInner),
  {
    let inner = Arc::get_mut(&mut self.inner).unwrap();
    f(inner);

    self
  }

  pub fn at<T>(&mut self, path: &str, service: T) -> &mut Self
  where
    T: HttpService + 'static,
  {
    self.tap_inner(|inner| {
      let insertion = inner.router.insert(path, Box::new(service));
      panic_err!(insertion, "Failed to insert route into router: {:#?}");
    })
  }
}

// struct HttpServiceFactory<'a>(pub &'a Resource);

// #[async_trait::async_trait]
// impl HttpService for HttpServiceFactory<'_> {
//   async fn call_service(&self, req: HttpRequest) -> HttpResponse {
//     let fut = self.0.run(req);
//     fut.await
//   }
// }

#[async_trait::async_trait]
impl HttpService for Resource {
  async fn call_service(&self, req: HttpRequest) -> HttpResponse {
    let fut = self.run(req);
    fut.await
  }
}

impl App {
  pub fn route<'a>(&'a self, path: &str) -> &Box<dyn HttpService + 'a> {
    let matched = match self.inner.router.at(path) {
      Ok(thing) => thing.value,
      Err(_) => &self.inner.default,
    };

    matched

    // Box::new(HttpServiceFactory(matched))
  }
}
