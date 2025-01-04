use std::{
  collections::{HashMap, HashSet},
  hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
  tag::{IntoTag, Tag},
  utils,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StyleRule {
  pub rule: String,
  pub styles: Vec<(String, String)>,
}

impl FromIterator<(String, String)> for StyleRule {
  fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
    let iter = iter.into_iter();

    let styles: Vec<(String, String)> = iter.collect();
    let mut hasher = DefaultHasher::default();

    styles.hash(&mut hasher);
    let key = hasher.finish();

    Self { rule: utils::encode_base62(key), styles }
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

#[derive(Default, Debug)]
pub struct Style {
  styles: HashSet<StyleRule>,
}

impl Style {
  pub fn add_rule(&mut self, style: StyleRule) {
    self.styles.insert(style);
  }
}

impl IntoTag for Style {
  fn into_tag(&self) -> Vec<Tag> {
    let mut styles = String::default();
    for item in &self.styles {
      styles.push_str(&item.to_string());
    }
    let tag = Tag::Tag {
      attributes: HashMap::default(),
      ident: "style",
      children: Some(Vec::from_iter([Tag::Text(styles)])),
      classes: Vec::default(),
      ids: Vec::default(),
    };
    Vec::from_iter([tag])
  }
}
