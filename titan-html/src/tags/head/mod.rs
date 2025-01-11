use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use crate::tags::{style::Style, IntoTag, Tag};

#[derive(Debug, Clone)]
pub struct Head {
  children: Vec<Tag>,
  opengraph: Option<opengraph::OpenGraph>,
}

impl IntoTag for Head {
  fn into_tag(mut self) -> Tag {
    if let Some(og) = &self.opengraph {
      self.opengraph_extend(og.clone());
    }
    Tag::Tag {
      ids: Vec::default(),
      children: Some(self.children),
      ident: "head",
      classes: HashSet::default(),
      attributes: HashMap::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    }
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
          classes: HashSet::default(),
          ids: Vec::default(),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
        Tag::Tag {
          ident: "meta",
          attributes: HashMap::from_iter([
            ("name".to_string(), "viewport".to_string()),
            (
              "content".to_string(),
              "width=device-width, initial-scale=1.0".to_string(),
            ),
          ]),
          children: None,
          classes: HashSet::default(),
          ids: Vec::default(),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
      ]),
    }
  }
}

impl Head {
  pub fn empty() -> Self {
    Self { opengraph: None, children: Vec::default() }
  }

  pub fn meta_utf8(mut self) -> Self {
    self.children.push(Tag::Tag {
      ident: "meta",
      attributes: HashMap::from_iter([(
        "charset".to_string(),
        "UTF-8".to_string(),
      )]),
      children: None,
      classes: HashSet::default(),
      ids: Vec::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    });
    self
  }

  pub fn title(mut self, title: &str) -> Self {
    self.children.push(Tag::Tag {
      ids: Vec::default(),
      ident: "title",
      children: Some(Vec::from_iter([Tag::Text(title.to_string())])),
      classes: HashSet::default(),
      attributes: HashMap::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    });
    self
  }

  pub fn append(mut self, tag: Tag) -> Self {
    self.children.push(tag);
    self
  }

  pub(crate) fn append_ref(&mut self, tag: Tag) {
    self.children.push(tag);
  }

  pub fn extend_ref<I>(&mut self, tags: I)
  where
    I: IntoIterator<Item = Tag>,
  {
    self.children.extend(tags);
  }

  pub fn global_style(mut self, style: Style) -> Self {
    self.children.push(style.into_tag());
    self
  }

  pub fn description(mut self, title: &str) -> Self {
    self.children.push(Tag::Tag {
      ids: Vec::default(),
      ident: "meta",
      children: None,
      classes: HashSet::default(),
      attributes: HashMap::from_iter([
        ("name".to_string(), "description".to_string()),
        ("content".to_string(), title.to_string()),
      ]),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    });
    self
  }

  pub fn opengraph(mut self, og: opengraph::OpenGraph) -> Self {
    self.opengraph = Some(og);
    self
  }

  pub fn reset_css(mut self) -> Self {
    let style =
      Style::from_str(include_str!("./reset.css")).unwrap().into_tag();
    self.children.push(style);
    self
  }
}

pub mod opengraph {
  use std::collections::{HashMap, HashSet};

  use crate::tags::Tag;

  use super::Head;

  #[derive(Debug, Clone)]
  pub struct OpenGraph {
    title: String,
    description: String,
    og_type: OpenGraphType,
    image_url: String,
  }

  impl OpenGraph {
    pub fn new(
      title: &str,
      description: &str,
      og_type: OpenGraphType,
      image_url: &str,
    ) -> Self {
      Self {
        title: title.to_string(),
        description: description.to_string(),
        og_type,
        image_url: image_url.to_string(),
      }
    }
  }

  impl Head {
    pub(super) fn opengraph_extend(&mut self, opengraph: OpenGraph) {
      let tags = Vec::from_iter([
        Tag::Tag {
          ident: "meta",
          ids: Vec::default(),
          children: None,
          classes: HashSet::default(),
          attributes: HashMap::from_iter([
            ("property".to_string(), "og:title".to_string()),
            ("content".to_string(), opengraph.title.clone()),
          ]),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
        Tag::Tag {
          ident: "meta",
          ids: Vec::default(),
          classes: HashSet::default(),
          children: None,
          attributes: HashMap::from_iter([
            ("property".to_string(), "og:description".to_string()),
            ("content".to_string(), opengraph.description.to_string()),
          ]),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
        Tag::Tag {
          ident: "meta",
          ids: Vec::default(),
          classes: HashSet::default(),
          children: None,
          attributes: HashMap::from_iter([
            ("property".to_string(), "og:type".to_string()),
            ("content".to_string(), opengraph.og_type.to_string()),
          ]),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
        Tag::Tag {
          ident: "meta",
          ids: Vec::default(),
          classes: HashSet::default(),
          children: None,
          attributes: HashMap::from_iter([
            ("property".to_string(), "og:image".to_string()),
            ("content".to_string(), opengraph.image_url.to_string()),
          ]),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        },
      ]);
      self.children.extend(tags);
    }
  }

  #[derive(Debug, Clone)]
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
}
