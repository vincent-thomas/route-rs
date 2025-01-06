#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::inherent_to_string)]
pub mod class;
mod context;
pub mod prelude;
mod stylerule;
pub mod tags;
mod utils;

pub use route_html_derive::css;

use context::Context;
use tags::{html::Html, IntoTag};

pub fn render(mut root: Html) -> String {
  let mut body = root.body.clone().into_tag();

  let mut ctx = Context::default();
  body.hydrate(&mut ctx);
  ctx.mutate_head(&mut root.head);

  let mut root_tag = root.into_tag();
  root_tag.children()[1] = body;

  format!("<!DOCTYPE html>{}", root_tag.to_string())
}
