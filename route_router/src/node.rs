use std::{cell::UnsafeCell, collections::HashMap, fmt::Debug};

use crate::RoutePart;

#[derive(Debug)]
pub struct Node<T> {
  children: HashMap<NodeMeta, Node<T>>,
  handler: Option<UnsafeCell<T>>,
  dynamic_child: Option<(String, Box<Node<T>>)>,
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
pub enum NodeMeta {
  #[default]
  Slash,
  StaticSegmentPart(char),
}

impl From<NodeMeta> for String {
  fn from(node: NodeMeta) -> String {
    match node {
      NodeMeta::Slash => "/".to_string(),
      NodeMeta::StaticSegmentPart(thing) => thing.to_string(),
    }
  }
}

impl<T> Node<T> {
  pub(crate) fn set_handler(&mut self, handler: T) {
    self.handler = Some(UnsafeCell::new(handler));
  }

  pub(crate) fn children_ref(&self) -> &HashMap<NodeMeta, Node<T>> {
    &self.children
  }

  pub(crate) fn children_mut(&mut self) -> &mut HashMap<NodeMeta, Node<T>> {
    &mut self.children
  }

  pub(crate) fn dynamic_child_ref(&self) -> &Option<(String, Box<Node<T>>)> {
    &self.dynamic_child
  }

  pub(crate) fn dynamic_child_mut(
    &mut self,
  ) -> &mut Option<(String, Box<Node<T>>)> {
    &mut self.dynamic_child
  }
}

impl<T: Debug> Node<T> {
  pub fn new() -> Self {
    Node { children: HashMap::new(), handler: None, dynamic_child: None }
  }

  pub fn internal_at(&self, path: &str) -> Option<*mut T> {
    let guidance = self.str_registrable(path);
    let mut node = self;
    let mut index = 0;

    let mut values = HashMap::new();

    let mut possible_dynamic_node: Option<Node<T>> = None;
    let mut possible_dynamic_current_node_value = String::new();
    let mut possible_dynamic_current_node_label = String::new();

    while index < guidance.len() {
      let node_meta = match &guidance[index] {
        RoutePart::Static(thing) => thing,
        RoutePart::Dynamic(_) => return None, // This case should never happen in lookup
      };

      if *node_meta == NodeMeta::Slash {
        values.insert(
          possible_dynamic_current_node_label.clone(),
          possible_dynamic_current_node_value.clone(),
        );
        possible_dynamic_node = None;
      }

      // Check for dynamic child
      if let Some((label, dynamic_child)) = &node.dynamic_child {
        possible_dynamic_node = Some(dynamic_child);
        possible_dynamic_current_node_label = label.clone();
      }

      // Check for static child
      if let Some(static_child) = node.children.get(node_meta) {
        if possible_dynamic_node.is_some() {
          let node_str: String = node_meta.clone().into();
          possible_dynamic_current_node_value.push_str(&node_str);
        }
        node = static_child;
        index += 1;
        continue;
      }
    }
    let dynamic_fallback = possible_dynamic_node.map(|v| v.handler)?;

    node.handler.or(dynamic_fallback).as_ref().map(|v| v.get())

    // let mut node = self;
    // let mut index = 0;
    //
    // let mut values = HashMap::new();
    //
    // loop {
    //   if index == guidance.len() {
    //     return node.handler.as_ref().map(|v| v.get());
    //   }
    //
    //   let node_meta = match &guidance[index] {
    //     RoutePart::Static(thing) => thing,
    //     RoutePart::Dynamic(_) => {
    //       unimplemented!("Dynamic segments doesn't happen on lookup")
    //     }
    //   };
    //
    //   // Go down the radix tree until a slash '/' is found. Then compare if the static param match,
    //   // otherwise use the dynamic route from the start of the slash before
    //
    //   if let Some((label, dynamic_child)) = node.dynamic_child_ref() {
    //     let mut node_clone = Some(node.clone());
    //     let mut possible_dynamic_value = "".to_string();
    //     let mut static_test_index = index;
    //     loop {
    //       let this_node = match node_clone {
    //         None => break,
    //         Some(node) => node,
    //       };
    //       if guidance[static_test_index] == RoutePart::Static(NodeMeta::Slash) {
    //         break;
    //       }
    //       let children = this_node.children_ref();
    //       if let Some(nice_thing) = children.get(node_meta) {
    //         let node_str: String = node_meta.clone().into();
    //         possible_dynamic_value.push_str(&node_str);
    //         node_clone = Some(nice_thing);
    //       } else {
    //         node_clone = None;
    //       }
    //       static_test_index += 1;
    //     }
    //     dbg!(&possible_dynamic_value);
    //     dbg!(&node_clone);
    //
    //     let real_node = match node_clone {
    //       None => dynamic_child,
    //       Some(value) => {
    //         values.insert(label, possible_dynamic_value);
    //         value
    //       }
    //     };
    //
    //     node = real_node;
    //   }
    //
    //   if let Some(nice_thing) = node.children.get(node_meta) {
    //     node = nice_thing;
    //   } else {
    //     return None;
    //   }
    //   index += 1;
    // }

    // let mut node_clone = Some(node.clone());
    // let mut possible_dynamic_value = "".to_string();
    //
    // let mut static_test_index = index;
    //
    // loop {
    //   let this_node = match node_clone {
    //     None => break,
    //     Some(node) => node,
    //   };
    //   dbg!(&this_node);
    //   if guidance[static_test_index] == RoutePart::Static(NodeMeta::Slash) {
    //     break;
    //   }
    //
    //   let children = this_node.children_ref();
    //
    //   if let Some(nice_thing) = children.get(node_meta) {
    //     let node_str: String = node_meta.clone().into();
    //     possible_dynamic_value.push_str(&node_str);
    //     node_clone = Some(nice_thing);
    //   } else {
    //     node_clone = None;
    //   }
    //
    //   static_test_index += 1;
    // }
    //
    // match node_clone {
    //   None => {
    //     println!("dynamic value = {}", possible_dynamic_value);
    //   }
    //   Some(value) => {
    //     println!("static value");
    //   }
    // };
    //
    // node = node_clone?;

    // if let Some((label, boxed_node)) = node.dynamic_child.as_ref() {
    //   let start_index = index;
    //   let end_index;
    //
    //   let node_testable_for_non_dynamic = node.clone();
    //
    //   loop {
    //     if guidance.len() - 1 == index
    //       || guidance[index] == RoutePart::Static(NodeMeta::Slash)
    //     {
    //       end_index = index;
    //       break;
    //     }
    //
    //     index += 1;
    //   }
    //   let dynamic_value = path[start_index..end_index + 1].to_string();
    //   values.insert(label.clone(), dynamic_value);
    // }

    //match &guidance[index] {
    // RoutePart::Static(thing) => {
    //   if let Some((label, boxed_node)) = node.dynamic_child.as_ref() {
    //     let start_index = index;
    //     let end_index;
    //
    //     loop {
    //       dbg!(&guidance);
    //       if guidance.len() - 1 == index
    //         || guidance[index] == RoutePart::Static(NodeMeta::Slash)
    //       {
    //         end_index = index;
    //         break;
    //       }
    //
    //       index += 1;
    //     }
    //
    //     let value = path[start_index..end_index + 1].to_string();
    //
    //     // Go down radix tree with a while loop to see if a static handler exists
    //     // if it does, we go down that path, if not, we insert the value into the hashmap
    //     let mut this_index = 0;
    //     let mut this_node = node;
    //     node = boxed_node;
    //     while this_index < value.len() {
    //       if let Some(nice_thing) =
    //         node.children.get(&NodeMeta::StaticSegmentPart(
    //           value.chars().nth(this_index).unwrap(),
    //         ))
    //       {
    //         this_node = nice_thing;
    //       } else {
    //         break;
    //       }
    //       this_index += 1;
    //     }
    //
    //     values.insert(label.clone(), value);
    //     if this_node.handler.is_some() {
    //       return this_node.handler.as_ref().map(|v| v.get());
    //     }
    //   }
    //
    //   if let Some(nice_thing) = node.children.get(thing) {
    //     node = nice_thing;
    //   }
    // }
    // RoutePart::Dynamic(_) => {
    //   unimplemented!("Dynamic segments doesn't happen on lookup")
    // }
    //}
  }

  pub fn at_mut(&self, path: &str) -> Option<&mut T> {
    let result = self.internal_at(path);
    result.map(|ptr| unsafe { ptr.as_mut() }.unwrap())
  }

  pub fn at(&self, path: &str) -> Option<&T> {
    let result = self.internal_at(path);
    result.map(|v| unsafe { v.as_ref() }.unwrap())
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
}
