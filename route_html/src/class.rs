#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum TagClass {
  /// Just a class
  Normal(String),
  /// Style that needs processing.
  Style(String),
}

impl TagClass {
  pub fn text(value: String) -> TagClass {
    Self::Normal(value)
  }

  pub fn styles(value: String) -> TagClass {
    Self::Style(value)
  }
}

//impl From<String> for TagClass {
//  fn from(value: String) -> Self {
//    Self::Style(value)
//  }
//}

//impl From<StyleRule> for TagClass {
//  fn from(value: StyleRule) -> Self {
//    TagClass::Style(value)
//  }
//}
