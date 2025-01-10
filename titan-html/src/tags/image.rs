use std::collections::{HashMap, HashSet};

use url::Url;

use super::{IntoTag, Tag};

titan_html_derive::html_tag! {
  pub struct Img {
    image_url: Url,
    alt: String,
    preload: bool
  }
}

impl Img {
  pub fn new(alt: &'static str, image_url: &'static str) -> Self {
    Self {
      preload: false,
      classes: HashSet::default(),
      image_url: Url::parse(image_url).expect("Invalid url"),
      alt: alt.to_string(),
      ids: Vec::default(),
      attributes: HashMap::default(),
    }
  }

  pub const fn preload(mut self) -> Self {
    self.preload = true;
    self
  }
}

impl IntoTag for Img {
  fn into_tag(self) -> Tag {
    let mut preconnect = HashSet::from_iter([]);

    if self.preload {
      preconnect.insert(format!(
        "{}://{}",
        self.image_url.scheme(),
        self.image_url.host_str().unwrap()
      ));
    }
    Tag::Tag {
      ident: "img",
      children: None,
      classes: self.classes,
      urls_to_preconnect: preconnect,
      ids: self.ids,
      attributes: HashMap::from_iter([
        ("src".to_string(), self.image_url.to_string()),
        ("alt".to_string(), self.alt),
      ]),
      urls_to_prefetch: HashSet::default(),
    }
  }
}
