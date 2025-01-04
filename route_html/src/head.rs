use std::collections::HashMap;

use crate::tag::{IntoTag, Tag};

#[derive(Debug)]
pub struct Head {
  pub children: Vec<Tag>,
  opengraph: Option<OpenGraph>,
}

#[derive(Debug)]
pub struct OpenGraph {
  title: String,
  description: String,
  og_type: OpenGraphType,
  og_url: String,
  image_url: String,
}

impl OpenGraph {
  pub fn new(
    title: &str,
    description: &str,
    og_type: OpenGraphType,
    og_url: &str,
    image_url: &str,
  ) -> Self {
    Self {
      title: title.to_string(),
      description: description.to_string(),
      og_type,
      og_url: og_url.to_string(),
      image_url: image_url.to_string(),
    }
  }
}

impl IntoTag for OpenGraph {
  fn into_tag(&self) -> Vec<Tag> {
    Vec::from_iter([
      Tag::Tag {
        ident: "meta",
        ids: Vec::default(),
        children: None,
        classes: Vec::default(),
        attributes: HashMap::from_iter([
          ("property".to_string(), "og:title".to_string()),
          ("content".to_string(), self.title.clone()),
        ]),
      },
      Tag::Tag {
        ident: "meta",
        ids: Vec::default(),
        classes: Vec::default(),
        children: None,
        attributes: HashMap::from_iter([
          ("property".to_string(), "og:description".to_string()),
          ("content".to_string(), self.description.to_string()),
        ]),
      },
      Tag::Tag {
        ident: "meta",
        ids: Vec::default(),
        classes: Vec::default(),
        children: None,
        attributes: HashMap::from_iter([
          ("property".to_string(), "og:type".to_string()),
          ("content".to_string(), self.og_type.to_string()),
        ]),
      },
      Tag::Tag {
        ident: "meta",
        ids: Vec::default(),
        classes: Vec::default(),
        children: None,
        attributes: HashMap::from_iter([
          ("property".to_string(), "og:image".to_string()),
          ("content".to_string(), self.image_url.to_string()),
        ]),
      },
    ])
  }
}

#[derive(Debug)]
pub enum OpenGraphType {
  Website,
  Article,
  VideoMovie,
  MusicSong,
}

impl ToString for OpenGraphType {
  fn to_string(&self) -> String {
    match self {
      OpenGraphType::Website => "website",
      OpenGraphType::Article => "article",
      OpenGraphType::VideoMovie => "video.movie",
      OpenGraphType::MusicSong => "music.song",
    }
    .to_string()
  }
}

impl IntoTag for Head {
  fn into_tag(&self) -> Vec<Tag> {
    let mut children = self.children.clone();
    if let Some(og) = &self.opengraph {
      children.extend(og.into_tag());
    }
    Vec::from_iter([Tag::Tag {
      ids: Vec::default(),
      children: Some(children),
      ident: "head",
      classes: Vec::default(),
      attributes: HashMap::default(),
    }])
  }
}

impl Head {
  pub fn title(mut self, title: &str) -> Self {
    self.children.push(Tag::Tag {
      ids: Vec::default(),
      ident: "title",
      children: Some(Vec::from_iter([Tag::Text(title.to_string())])),
      classes: Vec::default(),
      attributes: HashMap::default(),
    });
    self
  }

  pub fn description(mut self, title: &str) -> Self {
    self.children.push(Tag::Tag {
      ids: Vec::default(),
      ident: "meta",
      children: None,
      classes: Vec::default(),
      attributes: HashMap::from_iter([
        ("name".to_string(), "description".to_string()),
        ("content".to_string(), title.to_string()),
      ]),
    });
    self
  }

  pub fn opengraph(mut self, og: OpenGraph) -> Self {
    self.opengraph = Some(og);
    self
  }
}

impl Default for Head {
  fn default() -> Self {
    Head {
      opengraph: None,
      children: Vec::from_iter([
        Tag::Tag {
          ident: "meta",
          attributes: HashMap::from_iter([(
            "charset".to_string(),
            "UTF-8".to_string(),
          )]),
          children: None,
          classes: Vec::default(),
          ids: Vec::default(),
        },
        Tag::Tag {
          ident: "meta",
          attributes: HashMap::from_iter([
            ("name".to_string(), "viewport".to_string()),
            (
              "content".to_string(),
              "with=device-width, initial-scale=1.0".to_string(),
            ),
          ]),
          children: None,
          classes: Vec::default(),
          ids: Vec::default(),
        },
      ]),
    }
  }
}
