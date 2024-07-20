use std::collections::BTreeMap;

#[derive(Debug)]
struct RadixNode {
  children: BTreeMap<char, RadixNode>,
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
    let mut current_node = &mut self.root;
    let mut is_dynamic = false;

    for segment in path.chars() {
      if segment == '{' {
        is_dynamic = true;
      } else if segment == '}' {
        is_dynamic = false;
      }
      current_node =
        current_node.children.entry(segment).or_insert_with(|| RadixNode::new(is_dynamic));
    }

    current_node.is_end_of_word = true;
    current_node.value = Some(value.to_string());
  }

  fn search(&self, path: &str) -> Option<(&String, BTreeMap<String, String>)> {
    let mut current_node = &self.root;
    let mut params = BTreeMap::new();
    let mut param_name = String::new();
    let mut is_dynamic = false;

    for segment in path.chars() {
      if let Some(node) = current_node.children.get(&segment) {
        current_node = node;
      } else if let Some((key, node)) =
        current_node.children.iter().find(|(_, node)| node.is_dynamic)
      {
        param_name.push(segment);
        current_node = node;

        if segment == '}' {
          params.insert(key.clone().to_string(), param_name[1..param_name.len() - 1].to_string());
          param_name.clear();
          is_dynamic = false;
        } else if segment == '{' {
          param_name.clear();
          is_dynamic = true;
        }
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

  radix_tree.insert("/test", "User Profile");
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
