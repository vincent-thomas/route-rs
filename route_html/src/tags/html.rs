use std::collections::{HashMap, HashSet};

use super::head::Head;

use super::{Body, IntoTag, Tag};

#[derive(Debug, Default, Clone)]
pub struct Html {
  pub head: Head,
  pub body: Body,
}
impl Html {
  pub fn with_head(head: Head) -> Self {
    Html { head, body: Body::default() }
  }
  pub fn body_from_iter<T>(mut self, tags: T) -> Self
  where
    T: IntoIterator<Item = Tag>,
  {
    self.body.children.extend(tags);
    self
  }
}

impl From<(Head, Body)> for Html {
  fn from(value: (Head, Body)) -> Self {
    Html { head: value.0, body: value.1 }
  }
}

impl IntoTag for Html {
  fn into_tag(self) -> Tag {
    Tag::Tag {
      children: Some(vec![self.head.into_tag(), self.body.into_tag()]),
      classes: HashSet::default(),
      attributes: HashMap::default(),
      ident: "html",
      ids: Vec::default(),

      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    }
  }
}
