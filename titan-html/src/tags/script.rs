use std::collections::{HashMap, HashSet};

use url::Url;

use super::{IntoTag, Tag};

pub enum ScriptContent {
  Text(String),
  External(Url),
}

pub struct Script {
  content: ScriptContent,
  is_async: bool,
  is_defer: bool,
  is_modules: bool,
}

impl Script {
  fn from_scriptcontent(content: ScriptContent) -> Self {
    Script { content, is_async: false, is_modules: false, is_defer: false }
  }

  pub fn from_url(url: impl Into<String>) -> Self {
    let url_str: String = url.into();
    let url: Url =
      url_str.as_str().try_into().expect("Invalid url for external script");

    Self::from_scriptcontent(ScriptContent::External(url))
  }

  pub fn from_text(text: impl Into<String>) -> Self {
    Self::from_scriptcontent(ScriptContent::Text(text.into()))
  }

  pub fn set_async(mut self) -> Self {
    self.is_async = true;
  }

  pub fn set_defer(mut self) -> Self {
    self.is_defer = true;
  }

  pub fn set_module(mut self) -> Self {
    self.is_modules = true;
  }
}

impl IntoTag for Script {
  fn into_tag(self) -> Tag {
    let mut attributes = HashMap::default();

    if self.is_async {
      attributes.insert("async".to_string(), "".to_string());
    }

    if self.is_defer {
      attributes.insert("defer".to_string(), "".to_string());
    }

    if self.is_modules {
      attributes.insert("type".to_string(), "module".to_string());
    }

    match self.content {
      ScriptContent::Text(text) => Tag::Tag {
        ident: "script",
        children: Some([Tag::Text(text)].to_vec()),
        attributes,
        classes: HashSet::default(),
        ids: Vec::default(),
        urls_to_preconnect: HashSet::default(),
        urls_to_prefetch: HashSet::default(),
      },

      ScriptContent::External(url) => {
        attributes.insert("src".to_string(), url.to_string());
        Tag::Tag {
          ident: "script",
          children: None,
          attributes,
          ids: Vec::default(),
          urls_to_prefetch: HashSet::default(),
          urls_to_preconnect: HashSet::default(),
          classes: HashSet::default(),
        }
      }
    }
  }
}
