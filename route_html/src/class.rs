use crate::stylerule::StyleRule;

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
