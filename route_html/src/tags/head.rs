use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use crate::tags::{style::Style, IntoTag, Tag};

#[derive(Debug)]
pub struct Head {
  children: Vec<Tag>,
  opengraph: Option<opengraph::OpenGraph>,
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
      classes: HashSet::default(),
      attributes: HashMap::default(),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    }])
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

  pub fn extend_ref(&mut self, tags: Vec<Tag>) {
    self.children.extend(tags);
  }

  pub fn style(mut self, style: Style) -> Self {
    self.children.extend(style.into_tag());
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
    let style = Style::from_str(RESET_CSS).unwrap().into_tag();
    self.children.extend(style);
    self
  }
}

pub mod opengraph {
  use std::collections::{HashMap, HashSet};

  use crate::tags::{IntoTag, Tag};

  #[derive(Debug)]
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

  impl IntoTag for OpenGraph {
    fn into_tag(&self) -> Vec<Tag> {
      Vec::from_iter([
        Tag::Tag {
          ident: "meta",
          ids: Vec::default(),
          children: None,
          classes: HashSet::default(),
          attributes: HashMap::from_iter([
            ("property".to_string(), "og:title".to_string()),
            ("content".to_string(), self.title.clone()),
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
            ("content".to_string(), self.description.to_string()),
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
            ("content".to_string(), self.og_type.to_string()),
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
            ("content".to_string(), self.image_url.to_string()),
          ]),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
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
}

const RESET_CSS: &str = "
/*! modern-normalize v3.0.1 | MIT License | https://github.com/sindresorhus/modern-normalize */
*,
::before,
::after {
	box-sizing: border-box;
}

html {
	font-family:
		system-ui,
		'Segoe UI',
		Roboto,
		Helvetica,
		Arial,
		sans-serif,
		'Apple Color Emoji',
		'Segoe UI Emoji';
	line-height: 1.15;
	-webkit-text-size-adjust: 100%;
	tab-size: 4;
}

body {
	margin: 0;
}

b,
strong {
	font-weight: bolder;
}

code,
kbd,
samp,
pre {
	font-family:
		ui-monospace,
		SFMono-Regular,
		Consolas,
		'Liberation Mono',
		Menlo,
		monospace; /* 1 */
	font-size: 1em; /* 2 */
}

small {
	font-size: 80%;
}

sub,
sup {
	font-size: 75%;
	line-height: 0;
	position: relative;
	vertical-align: baseline;
}

sub {
	bottom: -0.25em;
}

sup {
	top: -0.5em;
}

table {
	border-color: currentcolor;
}

button,
input,
optgroup,
select,
textarea {
	font-family: inherit; /* 1 */
	font-size: 100%; /* 1 */
	line-height: 1.15; /* 1 */
	margin: 0; /* 2 */
}

button,
[type='button'],
[type='reset'],
[type='submit'] {
	-webkit-appearance: button;
}

legend {
	padding: 0;
}

progress {
	vertical-align: baseline;
}

::-webkit-inner-spin-button,
::-webkit-outer-spin-button {
	height: auto;
}

[type='search'] {
	-webkit-appearance: textfield; /* 1 */
	outline-offset: -2px; /* 2 */
}

::-webkit-search-decoration {
	-webkit-appearance: none;
}

::-webkit-file-upload-button {
	-webkit-appearance: button; /* 1 */
	font: inherit; /* 2 */
}

summary {
	display: list-item;
}
";
