use std::collections::HashMap;

use crate::segments::{FindSegmentResult, Segments};

#[derive(Clone)]
pub struct Router<V>
where
  V: Clone,
{
  routes: Vec<(Segments, V)>,
  lookup_cache: HashMap<Segments, V>,
}

impl<V> Default for Router<V>
where
  V: Clone,
{
  fn default() -> Self {
    Self { routes: Vec::default(), lookup_cache: HashMap::default() }
  }
}

#[derive(Debug, PartialEq)]
pub struct Match<V> {
  pub value: V,
  pub params: HashMap<String, String>,
}

impl<V> IntoIterator for Router<V>
where
  V: Clone,
{
  type Item = (Segments, V);
  type IntoIter = std::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    let mut vec: Vec<(Segments, V)> = self.lookup_cache.into_iter().collect();

    vec.extend(self.routes);
    vec.into_iter()
  }
}

impl<V> Router<V>
where
  V: Clone,
{
  pub fn at(&mut self, route: &str, handler: V) {
    let segments: Segments = route.to_string().into();

    if segments.num_params() == 0 {
      self.lookup_cache.insert(segments, handler);
    } else {
      self.routes.push((segments.clone(), handler));
    }
  }

  pub fn find(&mut self, route: &str) -> Option<Match<&V>> {
    let segments = Segments::from(route.to_string());

    if let Some(value) = self.lookup_cache.get(&segments) {
      return Some(Match { value, params: HashMap::default() });
    };

    for (key, value) in self.routes.iter() {
      if let FindSegmentResult::Match(params) = key.find(&segments.0) {
        return Some(Match { value, params });
      };
    }
    None
  }

  pub fn lookup(&self, route: &str) -> Option<Match<&V>> {
    let from_request = Segments::from(route.to_string());

    if let Some(value) = self.lookup_cache.get(&from_request) {
      return Some(Match { value, params: HashMap::default() });
    };

    for (contract, value) in &self.routes {
      if let FindSegmentResult::Match(params) = contract.find(&from_request.0) {
        return Some(Match { value, params });
      };
    }
    None
  }
  pub fn lookup_mut(&mut self, route: &str) -> Option<Match<&mut V>> {
    let segments = Segments::from(route.to_string());

    if let Some(value) = self.lookup_cache.get_mut(&segments) {
      return Some(Match { value, params: HashMap::default() });
    };

    for (key, value) in self.routes.iter_mut() {
      if let FindSegmentResult::Match(params) = key.find(&segments.0) {
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

    router.at("/test/testing2", "testing2".to_string());
    router.at("/test/:var/:fdshj", nice.clone());
    router.at("/test/:var", ":var".to_string());
    router.at("/test2", nice.clone());

    router.at("/nice", "nice2".to_string());

    if let Some(_static) = router.lookup("/test/testing2") {
      if _static.value == "testing2" {
        assert_eq!(_static.params.len(), 0);
      }
    };

    if let Some(_static) = router.lookup("/test/testing") {
      if _static.value == ":var" {
        assert_eq!(_static.params.len(), 1);
      }
    };

    assert_eq!(router.lookup("/te"), None); // BIG PROBLEM FATAL TODO:
    assert!(router.lookup("/ni").is_none()); // BIG PROBLEM FATAL TODO:
  }
}
