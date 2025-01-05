use std::collections::HashMap;

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
  pub fn body_from_iter<T, In>(mut self, tags: T) -> Self
  where
    T: IntoIterator<Item = In>,
    In: IntoTag,
  {
    let tags: Vec<Tag> = tags.into_iter().flat_map(|x| x.into_tag()).collect();
    self.body.children.extend(tags);
    self
  }
}

impl IntoTag for Html {
  fn into_tag(&self) -> Vec<Tag> {
    let mut children = Vec::default();

    children.extend(self.head.into_tag());
    children.extend(self.body.into_tag());

    Vec::from_iter([
      Tag::Text("<!DOCTYPE html>".to_string()),
      Tag::Tag {
        children: Some(children),
        classes: Vec::default(),
        attributes: HashMap::default(),
        ident: "html",
        ids: Vec::default(),
      },
    ])
  }
}
