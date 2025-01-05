use std::{
  collections::{HashMap, HashSet},
  hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
  tags::{IntoTag, Tag},
  utils,
};

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

    let mut rule = String::from("r");
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

#[derive(Debug)]
pub enum Style {
  Styles(HashSet<StyleRule>),
  Text(String),
}

impl Default for Style {
  fn default() -> Self {
    Style::Styles(HashSet::default())
  }
}

impl Style {
  pub fn add_rule(&mut self, style: StyleRule) {
    match self {
      Style::Styles(styles) => {
        styles.insert(style);
      }
      Style::Text(text) => text.push_str(style.to_string().as_str()),
    };
  }
}

impl IntoTag for Style {
  fn into_tag(&self) -> Vec<Tag> {
    let content = match self {
      Style::Text(text) => text.clone(),
      Style::Styles(styles) => {
        styles.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
      }
    };

    let tag = Tag::Tag {
      attributes: HashMap::default(),
      ident: "style",
      children: Some(Vec::from_iter([Tag::Text(content)])),
      classes: Vec::default(),
      ids: Vec::default(),
    };
    Vec::from_iter([tag])
  }
}

impl<T> From<T> for Style
where
  T: Into<String>,
{
  fn from(value: T) -> Self {
    let str = value.into();

    Style::Text(str)
  }
}
