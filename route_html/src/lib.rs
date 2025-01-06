#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::inherent_to_string)]
pub mod class;
mod stylerule;
pub mod tags;
mod utils;

use std::collections::{HashMap, HashSet};

use stylerule::StyleRule;
use tags::{html::Html, style::Style, IntoTag, Tag};

#[derive(Default)]
pub(crate) struct Context {
  styles: Style,
  other_tags: Vec<Tag>,
}

impl Context {
  pub fn add_styles(&mut self, stylerule: StyleRule) {
    self.styles.add_rule(stylerule);
  }

  pub fn add_preconnect(&mut self, url: String) {
    self.other_tags.push(Tag::Tag {
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
    })
  }

  pub fn add_prefetch(&mut self, url: String) {
    self.other_tags.push(Tag::Tag {
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
    })
  }
}

impl IntoTag for Context {
  fn into_tag(&self) -> Vec<tags::Tag> {
    let mut vec = self.styles.into_tag();
    vec.extend(self.other_tags.clone());
    vec
  }
}

pub fn render(mut root: Html) -> String {
  let mut body = root.body.into_tag()[0].clone();

  let mut ctx = Context::default();

  body.hydrate(&mut ctx);

  root.head.extend_ref(ctx.into_tag());

  let mut root_tag = root.into_tag();

  root_tag[1].children()[1] = body;

  let mut document = root_tag[0].to_string();

  document.push_str(root_tag[1].to_string().as_str());

  document
}
