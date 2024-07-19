use std::sync::Arc;

use matchit::Router;

use crate::{panic_err, resource::Resource};

#[derive(Default)]
pub struct App {
  inner: Arc<AppInner>,
  // router: Router<Resource>,
  // default: Resource,
}

// unsafe impl Send for App {}
// unsafe impl Sync for App {}

#[derive(Default)]
struct AppInner {
  pub router: Router<Resource>,
  pub default: Resource,
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

  pub fn at(&mut self, path: &str, service: Resource) -> &mut Self {
    self.tap_inner(|inner| {
      let insertion = inner.router.insert(path, service);
      panic_err!(insertion, "Failed to insert route into router: {:#?}");
    })
  }
}

// impl FindableRoute for App {
//   fn find_route(&self, path: &str) -> Box<dyn HttpService + '_> {
//     let matched = match self.inner.router.at(path) {
//       Ok(thing) => thing.value,
//       Err(_) => &self.inner.default,
//     };

//     Box::new(HttpServiceFactory(matched.route_ref()))
//   }
// }
