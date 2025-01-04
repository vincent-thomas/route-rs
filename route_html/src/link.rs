use std::collections::HashMap;

use crate::{
  class::TagClass,
  tag::{IntoTag, Tag},
};

#[derive(Default)]
pub enum LinkTarget {
  #[default]
  _Self,
  _Blank,
  // Other
}

impl ToString for LinkTarget {
  fn to_string(&self) -> String {
    match self {
      LinkTarget::_Blank => "_blank".to_string(),
      LinkTarget::_Self => "_self".to_string(),
    }
  }
}

#[derive(Default)]
pub enum LinkLoadType {
  #[default]
  No,
  WhenIdle,
}

pub struct Link {
  pub children: Vec<Tag>,
  pub classes: Vec<TagClass>,
  pub ids: Vec<String>,

  href: String,
  target: LinkTarget,

  pub load_type: LinkLoadType,
}

impl IntoTag for Link {
  fn into_tag(&self) -> Vec<Tag> {
    let tag = Tag::Tag {
      classes: self.classes.clone(),
      ids: self.ids.clone(),
      ident: "a",
      children: Some(self.children.clone()),
      attributes: HashMap::from_iter([
        ("href".to_string(), self.href.to_string()),
        ("target".to_string(), self.target.to_string()),
      ]),
    };

    let mut tags = Vec::from_iter([tag]);

    match self.load_type {
      LinkLoadType::No => tags,
      LinkLoadType::WhenIdle => {
        let tag = Tag::Tag {
          ident: "link",
          ids: Vec::default(),
          children: None,
          classes: Vec::default(),
          attributes: HashMap::from_iter([
            ("rel".to_string(), "preload".to_string()),
            ("href".to_string(), self.href.clone()),
          ]),
        };

        tags.push(tag);
        tags
      }
    }
  }
}

impl Link {
  pub fn new(href: String, children: Vec<Box<dyn IntoTag>>) -> Self {
    Self {
      href,
      children: children
        .into_iter()
        .flat_map(|x| x.into_tag().clone())
        .collect(),
      classes: vec![],
      ids: vec![],
      load_type: LinkLoadType::default(),
      target: LinkTarget::default(),
    }
  }

  pub fn preload(mut self, preload: LinkLoadType) -> Self {
    self.load_type = preload;
    self
  }
}
