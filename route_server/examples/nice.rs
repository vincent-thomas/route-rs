use std::collections::BTreeMap;

#[derive(Debug)]
struct RadixNode {
  children: BTreeMap<String, RadixNode>,
  is_end_of_word: bool,
  value: Option<String>,
  is_dynamic: bool,
}

impl RadixNode {
  fn new(is_dynamic: bool) -> Self {
    RadixNode { children: BTreeMap::new(), is_end_of_word: false, value: None, is_dynamic }
  }
}

#[derive(Debug)]
struct RadixTree {
  root: RadixNode,
}

impl RadixTree {
  fn new() -> Self {
    RadixTree { root: RadixNode::new(false) }
  }

  fn insert(&mut self, path: &str, value: &str) {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut current_node = &mut self.root;

    for segment in segments {
      let is_dynamic = segment.starts_with('{') && segment.ends_with('}');
      current_node = current_node
        .children
        .entry(segment.to_string())
        .or_insert_with(|| RadixNode::new(is_dynamic));
    }

    current_node.is_end_of_word = true;
    current_node.value = Some(value.to_string());
  }

  fn search(&self, path: &str) -> Option<(&String, BTreeMap<String, String>)> {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut current_node = &self.root;
    let mut params = BTreeMap::new();

    for segment in segments {
      if let Some(node) = current_node.children.get(segment) {
        current_node = node;
      } else if let Some((key, node)) =
        current_node.children.iter().find(|(_, node)| node.is_dynamic)
      {
        let param_name = &key[1..key.len() - 1];
        params.insert(param_name.to_string(), segment.to_string());
        current_node = node;
      } else {
        return None;
      }
    }

    if current_node.is_end_of_word {
      Some((current_node.value.as_ref().unwrap(), params))
    } else {
      None
    }
  }
}

fn main() {
  let mut radix_tree = RadixTree::new();

  radix_tree.insert("/test/{user_id}", "User Profile");
  radix_tree.insert("/test/{user_id}/settings", "User Settings");
  radix_tree.insert("/products/{product_id}", "Product Details");

  if let Some((value, params)) = radix_tree.search("/test/123") {
    println!("Value: {}, Params: {:?}", value, params); // Output: Value: User Profile, Params: {"user_id": "123"}
  }

  if let Some((value, params)) = radix_tree.search("/test/123/settings") {
    println!("Value: {}, Params: {:?}", value, params); // Output: Value: User Settings, Params: {"user_id": "123"}
  }

  if let Some((value, params)) = radix_tree.search("/products/456") {
    println!("Value: {}, Params: {:?}", value, params); // Output: Value: Product Details, Params: {"product_id": "456"}
  }
}
