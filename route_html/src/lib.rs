#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::inherent_to_string)]
pub mod class;
pub mod head;
pub mod link;
pub mod style;
pub mod tag;
mod utils;

use style::Style;
use tag::{Html, IntoTag};

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
