use std::hash::{DefaultHasher, Hash, Hasher};

use crate::utils;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StyleRule {
  pub rule: String,
  pub styles: Vec<(String, String)>,
}

impl<'a> FromIterator<(&'a str, &'a str)> for StyleRule {
  fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
    let styles: Vec<(String, String)> = iter
      .into_iter()
      .map(|(key, value)| (key.to_string(), value.to_string()))
      .collect();
    let mut hasher = DefaultHasher::default();

    styles.hash(&mut hasher);
    let key = hasher.finish();

    let mut rule = String::with_capacity(64);
    rule.push('r'); // CSS-rules cannot start with a number
    rule.push_str(&utils::encode_base62(key));

    Self { rule, styles }
  }
}

impl FromIterator<(String, String)> for StyleRule {
  fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
    let styles: Vec<(String, String)> = iter.into_iter().collect();
    let mut hasher = DefaultHasher::default();

    styles.hash(&mut hasher);
    let key = hasher.finish();
    let mut rule = String::from("r");
    rule.push_str(&utils::encode_base62(key));

    Self { rule, styles }
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
