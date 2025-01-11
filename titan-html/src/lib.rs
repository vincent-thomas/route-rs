#![allow(clippy::to_string_trait_impl)]
#![allow(clippy::inherent_to_string)]
pub mod class;
mod context;
pub mod prelude;
pub mod tags;

pub use titan_html_core::*;

pub use titan_html_derive::{css, global_css};

use context::Context;
use tags::{html::Html, Body, IntoTag};

const DOCTYPE: &str = "<!DOCTYPE html>";

pub fn render(root: Html) -> String {
  let mut body = root.body.into_tag();
  let mut context = Context::from(root.head);
  body.hydrate(&mut context);

  let mut html = Html {
    head: context.into(),
    // Removed by line below
    body: Body::default(),
  }
  .into_tag();
  html.children()[1] = body;

  format!("{}{}", DOCTYPE, html.to_string())
}
