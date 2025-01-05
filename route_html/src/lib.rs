#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::inherent_to_string)]
pub mod class;
pub mod tags;
mod utils;

use tags::{html::Html, style::Style, IntoTag};

pub fn render(mut root: Html) -> String {
  let body = root.body.into_tag();

  let mut styles = Style::default();

  body[0].hydrate_styles(&mut styles);

  root.head.children.extend(styles.into_tag());

  let root_tag = root.into_tag();

  let mut document = root_tag[0].to_string();

  document.push_str(root_tag[1].to_string().as_str());

  document
}
