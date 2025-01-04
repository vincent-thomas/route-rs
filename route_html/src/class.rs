use crate::style::StyleRule;

#[derive(Clone, Debug)]
pub enum TagClass {
  Normal(String),
  Style(StyleRule),
}

impl From<String> for TagClass {
  fn from(value: String) -> Self {
    Self::Normal(value)
  }
}

impl From<StyleRule> for TagClass {
  fn from(value: StyleRule) -> Self {
    TagClass::Style(value)
  }
}

//#[cfg(test)]
//mod tests {
//  use super::*;
//  #[test]
//  fn tests() {
//    let tesing = "
//            color: white;
//            background-color: black;
//        ";
//
//    let style = StyleRule::from_str(tesing).unwrap();
//
//    let facit = Vec::from_iter([
//      ("color".to_string(), "white".to_string()),
//      ("background-color".to_string(), "black".to_string()),
//    ]);
//
//    assert_eq!(StyleRule { rule: "".to_string(), styles: facit }, style);
//  }
//}
