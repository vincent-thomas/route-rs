use std::{collections::HashMap, fmt::Debug};

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
        nodes.push(RoutePart::Static(NodeMeta::StaticSegmentPart(path_vec[index])));
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
          node = node.children.entry(thing).or_insert_with(Node::new);
        }
        RoutePart::Dynamic(thing) => {
          if node.dynamic_child.is_none() {
            node.dynamic_child = Some((thing, Box::new(Node::new())));
          } // node.dynamic_child.[... ].as_mut(); is this ^
          node = node.dynamic_child.as_mut().unwrap().1.as_mut();
        }
      }
    }

    node.handler = Some(handler);
  }

  pub fn at(&self, path: &str) -> Option<&T>
  where
    T: Debug,
  {
    self.tree.at(path)
    // let guidance = self.str_registrable(path);

    // let mut node = &self.tree;
    // let mut index = 0;

    // let mut values = HashMap::new();

    // loop {
    //   if index == guidance.len() {
    //     return node.handler.as_ref();
    //   }

    //   match &guidance[index] {
    //     RoutePart::Static(thing) => {
    //       dbg!(&thing);
    //       if let Some((label, boxed_node)) = node.dynamic_child.as_ref() {
    //         let start_index = index;
    //         let end_index;

    //         loop {
    //           dbg!(&guidance[index], index);

    //           if guidance.len() - 1 == index
    //             || guidance[index] == RoutePart::Static(NodeMeta::Slash)
    //           {
    //             end_index = index;
    //             break;
    //           }

    //           index += 1;
    //         }

    //         let value = path[start_index..end_index + 1].to_string();

    //         // if let Some(handler_from_value) = node.children.get(NodeMeta::StaticSegmentPart(value))
    //         // {
    //         //   node = handler_from_value;
    //         // } else {
    //         values.insert(label.clone(), value);
    //         node = boxed_node.as_ref()
    //         // }
    //       }

    //       if let Some(nice_thing) = node.children.get(thing) {
    //         node = nice_thing;
    //       }
    //     }
    //     RoutePart::Dynamic(_) => unimplemented!("Dynamic segments doesn't happen on lookup"),
    //   }
    //   index += 1;
    // }
  }
  pub fn at_mut<'a>(&'a mut self, path: &str) -> Option<&'a mut T>
  where
    T: Debug,
  {
    let guidance = self.str_registrable(path);
    let mut node = &mut self.tree;
    let mut index = 0;
    let mut values = HashMap::new();

    while index < guidance.len() {
      match &guidance[index] {
        RoutePart::Static(thing) => {
          if let Some(dynamic_child) = node.dynamic_child.as_mut() {
            // Handle dynamic child
            let start_index = index;
            let mut end_index = start_index;

            while end_index < guidance.len() - 1
              && guidance[end_index] != RoutePart::Static(NodeMeta::Slash)
            {
              end_index += 1;
            }

            let value = &path[start_index..=end_index];

            if let Some(handler_from_value) = node.children.get_mut(thing) {
              node = handler_from_value;
            } else {
              let boxed_node = dynamic_child.1.as_mut();
              values.insert(dynamic_child.0.clone(), value.to_string());
              node = boxed_node;
            }
          } else if let Some(nice_thing) = node.children.get_mut(thing) {
            // Handle static child
            node = nice_thing;
          } else {
            return None;
          }
        }
        RoutePart::Dynamic(_) => {
          unimplemented!("Dynamic segments do not occur during lookup");
        }
      }
      index += 1;
    }

    node.handler.as_mut()
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

    // dbg!(&router);

    //assert_eq!(router.at("/hello/test2"), Some(&"Hello, world!"));
    assert_eq!(router.at("/hello/test"), Some(&"Hello, world!2"));
  }
}
