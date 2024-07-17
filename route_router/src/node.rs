use std::{collections::HashMap, fmt::Debug};

use crate::RoutePart;

#[derive(Debug)]
pub struct Node<T> {
  pub children: HashMap<NodeMeta, Node<T>>,
  pub handler: Option<T>,
  pub dynamic_child: Option<(String, Box<Node<T>>)>,
}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
pub enum NodeMeta {
  #[default]
  Slash,
  StaticSegmentPart(char),
}

impl<T> Node<T> {
  pub fn new() -> Self {
    Node { children: HashMap::new(), handler: None, dynamic_child: None }
  }

  pub fn at(&self, path: &str) -> Option<&T>
  where
    T: Debug,
  {
    let guidance = self.str_registrable(path);

    let mut node = self;
    let mut index = 0;

    let mut values = HashMap::new();

    loop {
      if index == guidance.len() {
        return node.handler.as_ref();
      }

      match &guidance[index] {
        RoutePart::Static(thing) => {
          if let Some((label, boxed_node)) = node.dynamic_child.as_ref() {
            let start_index = index;
            let end_index;

            loop {
              // dbg!(&guidance[index], index);

              if guidance.len() - 1 == index
                || guidance[index] == RoutePart::Static(NodeMeta::Slash)
              {
                end_index = index;
                break;
              }

              index += 1;
            }

            let value = path[start_index..end_index + 1].to_string();

            // Go down radix tree with a while loop to see if a static handler exists
            // if it does, we go down that path, if not, we insert the value into the hashmap
            let mut this_index = 0;
            let mut this_node = node;
            node = boxed_node;
            while this_index < value.len() {
              if let Some(nice_thing) = node
                .children
                .get(&NodeMeta::StaticSegmentPart(value.chars().nth(this_index).unwrap()))
              {
                this_node = nice_thing;
                dbg!(this_node);
              } else {
                dbg!("breaking", this_index, &value);
                break;
              }
              this_index += 1;
            }

            values.insert(label.clone(), value);
            if this_node.handler.is_some() {
              return this_node.handler.as_ref();
            }
          }

          if let Some(nice_thing) = node.children.get(thing) {
            node = nice_thing;
          }
        }
        RoutePart::Dynamic(_) => unimplemented!("Dynamic segments doesn't happen on lookup"),
      }
      index += 1;
    }
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
}
