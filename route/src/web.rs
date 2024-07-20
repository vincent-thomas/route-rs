use crate::resource::{Resource, Route};

#[cfg(feature = "types")]
pub use crate::types::*;

pub fn resource() -> Resource {
  Resource::default()
}

pub fn route() -> Route {
  Route::default()
}

#[cfg(feature = "types")]
pub fn redirect(path: &'static str) -> Redirect {
  Redirect::new(path)
}
