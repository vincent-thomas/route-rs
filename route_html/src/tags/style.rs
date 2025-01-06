use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use lightningcss::{
  printer::PrinterOptions,
  stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
};

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

  pub fn external(link: impl Into<String>) -> Tag {
    Tag::Tag {
      ids: Vec::default(),
      ident: "link",
      children: None,
      classes: HashSet::default(),
      attributes: HashMap::from_iter([
        ("rel".to_string(), "stylesheet".to_string()),
        ("href".to_string(), link.into()),
      ]),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    }
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
      classes: HashSet::default(),
      ids: Vec::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    };
    Vec::from_iter([tag])
  }
}

impl FromStr for Style {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut stylesheet =
      StyleSheet::parse(s, ParserOptions::default()).unwrap();

    stylesheet.minify(MinifyOptions::default()).unwrap();

    let style =
      Style::Text(stylesheet.to_css(PrinterOptions::default()).unwrap().code);

    Ok(style)
  }
}
