use std::collections::{HashMap, HashSet};

use super::head::Head;

use super::{Body, IntoTag, Tag};

#[derive(Debug, Default)]
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
    T: IntoIterator<Item = Vec<Tag>>,
  {
    let tags = tags.into_iter().flatten();
    self.body.children.extend(tags);
    self
  }
}

impl IntoTag for Html {
  fn into_tag(&self) -> Vec<Tag> {
    let mut children = self.head.into_tag();
    children.extend(self.body.into_tag());

    Vec::from_iter([
      Tag::Text("<!DOCTYPE html>".to_string()),
      Tag::Tag {
        children: Some(children),
        classes: HashSet::default(),
        attributes: HashMap::default(),
        ident: "html",
        ids: Vec::default(),

        urls_to_preconnect: HashSet::default(),
        urls_to_prefetch: HashSet::default(),
      },
    ])
  }
}
