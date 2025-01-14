use std::collections::{HashMap, HashSet};

use titan_html_core::StyleRule;

use crate::tags::{head::Head, IntoTag, Style, Tag};

#[derive(Default)]
pub(crate) struct Context {
  styles: Style,
  old_head: Head,
  other_tags: HashSet<Tag>,
}

impl From<Context> for Head {
  fn from(value: Context) -> Self {
    let mut head = value.old_head;

    head.append_ref(value.styles.into_tag());

    for item in value.other_tags {
      head.append_ref(item);
    }
    head
  }
}

impl From<Head> for Context {
  fn from(value: Head) -> Self {
    Context {
      old_head: value,
      other_tags: HashSet::default(),
      styles: Style::default(),
    }
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
