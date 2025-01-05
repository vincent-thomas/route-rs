use std::collections::{HashMap, HashSet};

use crate::{
  stylerule::StyleRule,
  tags::{IntoTag, Tag},
};

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
