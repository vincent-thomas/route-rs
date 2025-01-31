use titan_html_core::StyleRule;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum TagClass {
  /// Just a class
  Normal(String),
  StyleRule(StyleRule),
}

impl TagClass {
  pub fn text(value: String) -> TagClass {
    Self::Normal(value)
  }

  pub fn styles(value: StyleRule) -> TagClass {
    Self::StyleRule(value)
  }
}
