use std::collections::{HashMap, VecDeque};

use crate::{
  routekey::{Params, Segments},
  utils,
};

pub(crate) struct RouteId(usize);
pub struct Router<V> {
  routes: VecDeque<(Segments, V)>,
  cache: HashMap<Segments, RouteId>,
}

impl<V> Default for Router<V> {
  fn default() -> Self {
    Self { routes: VecDeque::default(), cache: HashMap::default() }
  }
}

pub struct Match<V> {
  pub value: V,
  pub params: HashMap<String, String>,
}

impl<V> Router<V> {
  pub fn at(&mut self, route: &str, handler: V) {
    let segments: Segments = route.to_string().into();

    let priority = segments.num_params();

    let index = self
      .routes
      .iter()
      .position(|x| x.0.num_params() <= priority)
      .unwrap_or(0);

    utils::insert_at(&mut self.routes, index, (segments.clone(), handler));
  }

  pub fn find(&mut self, route: &str) -> Option<Match<&V>> {
    let segments = Segments::from(route.to_string());

    if let Some(RouteId(route_index)) = self.cache.get(&segments) {
      let (_, value) = &self.routes[*route_index];
      return Some(Match { value, params: HashMap::default() });
    };

    for (index, (key, value)) in self.routes.iter().enumerate() {
      if let Some(Params(params)) = key.find(&segments.0) {
        if params.is_empty() {
          self.cache.insert(segments, RouteId(index));
        }
        return Some(Match { value, params });
      };
    }
    None
  }
  pub fn at_mut(&mut self, route: &str) -> Option<Match<&mut V>> {
    let segments = Segments::from(route.to_string());

    if let Some(RouteId(route_index)) = self.cache.get_mut(&segments) {
      let (_, value) = &mut self.routes[*route_index];
      return Some(Match { value, params: HashMap::default() });
    };

    for (index, (key, value)) in self.routes.iter_mut().enumerate() {
      if let Some(Params(params)) = key.find(&segments.0) {
        if params.is_empty() {
          self.cache.insert(segments, RouteId(index));
        }
        return Some(Match { value, params });
      };
    }
    None
  }
}

#[cfg(test)]
mod lib_tests {
  use super::*;
  #[test]
  fn test() {
    let mut router = Router::default();

    let nice = "nice".to_string();

    router.at("/test", nice.clone());
    router.at("/test/:var/:fdshj", nice.clone());
    router.at("/test/:var", nice.clone());
    router.at("/test2", nice.clone());
  }
}
