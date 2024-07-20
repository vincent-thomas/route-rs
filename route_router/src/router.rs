use std::fmt::Debug;

use crate::node::{Node, NodeMeta};

#[derive(Debug)]
pub struct Router<T> {
  tree: Node<T>,
}

impl<T> Default for Router<T> {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, PartialEq)]
pub enum RoutePart {
  Static(NodeMeta),
  Dynamic(String),
}

impl<T> Router<T> {
  pub fn new() -> Router<T> {
    Router { tree: Node::new() }
  }

  fn str_registrable(&self, path: &str) -> Vec<RoutePart> {
    // Transform path into a vec of "NodeMeta" enums
    let mut nodes: Vec<RoutePart> = vec![];
    let mut index = 0; // Denna kommer att inkrementas från början av loopen så man kan continuea;

    let path_vec = path.chars().collect::<Vec<char>>();

    loop {
      if index == path_vec.len() {
        return nodes;
      }
      if path_vec[index] == '/' {
        nodes.push(RoutePart::Static(NodeMeta::Slash));
      } else if path_vec[index] == '{' {
        let mut name = String::new();
        while path_vec[index] != '}' {
          name.push(path_vec[index]);
          index += 1;
        }
        nodes.push(RoutePart::Dynamic(name[1..].to_string()));
      } else {
        nodes.push(RoutePart::Static(NodeMeta::StaticSegmentPart(
          path_vec[index],
        )));
      }
      index += 1;
    }
  }

  pub fn route(&mut self, path: &str, handler: T) {
    let test = self.str_registrable(path);
    let mut node = &mut self.tree;

    for thing in test {
      match thing {
        RoutePart::Static(thing) => {
          node = node.children_mut().entry(thing).or_insert_with(Node::new);
        }
        RoutePart::Dynamic(thing) => {
          if node.dynamic_child_ref().is_none() {
            *node.dynamic_child_mut() = Some((thing, Box::new(Node::new())));
          } // node.dynamic_child.[... ].as_mut(); is this ^
          node = node.dynamic_child_mut().as_mut().unwrap().1.as_mut();
        }
      }
    }

    node.set_handler(handler);
  }

  pub fn at(&self, path: &str) -> Option<&T> {
    self.tree.at(path)
  }
  pub fn at_mut<'a>(&'a mut self, path: &str) -> Option<&'a mut T> {
    self.tree.at_mut(path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_router() {
    let mut router = Router::new();
    // router.route("/hello", "Hello, world!");
    router.route("/hello/{test}", "Hello, world!");
    router.route("/hello/test", "Hello, world!2");
    // router.route("/helflo", "Hello, world!");
    // router.route("/hello/est", "Hello, world!");

    //assert_eq!(router.at("/hello/test2"), Some(&"Hello, world!"));
    assert_eq!(router.at("/hello/test"), Some(&"Hello, world!2"));
  }
}
