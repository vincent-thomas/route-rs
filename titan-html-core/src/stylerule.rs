use std::hash::{DefaultHasher, Hash, Hasher};

use crate::utils;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StyleRule {
  pub rule: String,
  pub styles: Vec<(Box<str>, Box<str>)>,
}

impl<'a> FromIterator<(&'a str, &'a str)> for StyleRule {
  fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
    let mut hasher = DefaultHasher::default();

    let vec = iter
      .into_iter()
      .map(|x| (x.0.into(), x.1.into()))
      .collect::<Vec<(Box<str>, Box<str>)>>();

    vec.hash(&mut hasher);
    let key = hasher.finish();

    let mut rule = String::with_capacity(64);
    rule.push('r'); // CSS-rules cannot start with a number
    rule.push_str(&utils::encode_base62(key));

    Self { rule, styles: vec }
  }
}

impl ToString for StyleRule {
  fn to_string(&self) -> String {
    let styles = self
      .styles
      .iter()
      .map(|(key, value)| format!("{key}:{value};"))
      .collect::<Vec<String>>()
      .join("");

    let total = format!(".{}{{{}}}", self.rule, styles);

    total
  }
}
