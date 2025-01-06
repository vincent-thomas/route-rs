use std::collections::{HashMap, HashSet};

use crate::{
  stylerule::StyleRule,
  tags::{head::Head, style::Style, IntoTag, Tag},
};

#[derive(Default)]
pub(crate) struct Context {
  styles: Style,
  other_tags: HashSet<Tag>,
}

impl Context {
  pub(crate) fn mutate_head(self, head: &mut Head) {
    let style_tag = self.styles.into_tag().clone();
    head.append_ref(style_tag);
    head.extend_ref(self.other_tags);
  }
}

impl Context {
  pub fn add_styles(&mut self, stylerule: StyleRule) {
    self.styles.add_rule(stylerule);
  }

  pub fn add_preconnect(&mut self, url: String) {
    self.other_tags.insert(Tag::Tag {
      ident: "link",
      attributes: HashMap::from_iter([
        ("rel".to_string(), "preconnect".to_string()),
        ("href".to_string(), url),
      ]),
      children: None,
      classes: HashSet::default(),
      ids: Vec::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    });
  }

  pub fn add_prefetch(&mut self, url: String) {
    self.other_tags.insert(Tag::Tag {
      ident: "link",
      attributes: HashMap::from_iter([
        ("rel".to_string(), "prefetch".to_string()),
        ("href".to_string(), url),
      ]),
      children: None,
      classes: HashSet::default(),
      ids: Vec::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    });
  }
}

//impl IntoTag for Context {
//  fn into_tag(&self) -> Vec<tags::Tag> {
//    let mut vec = self.styles.into_tag();
//    vec.extend(self.other_tags.clone());
//    vec
//  }
//}
