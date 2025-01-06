use rand::Rng as _;
use std::collections::{HashMap, HashSet};

use crate::tags::{IntoTag, Tag};

#[derive(Default, Clone, Copy)]
pub enum LinkTarget {
  #[default]
  _Self,
  _Blank,
  // Other
}

impl ToString for LinkTarget {
  fn to_string(&self) -> String {
    match self {
      LinkTarget::_Blank => "_blank".to_string(),
      LinkTarget::_Self => "_self".to_string(),
    }
  }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum LinkLoadType {
  #[default]
  No,
  WhenIdle,
  WhenHover,
}

route_html_derive::html_tag! {
  pub struct Link {
    pub children: Vec<Tag>,
    href: String,
    target: LinkTarget,
    pub load_type: LinkLoadType,
  }
}

fn generate_random_letters(length: usize) -> String {
  let mut rng = rand::thread_rng();
  let mut random_string = String::new();

  // Define the alphabet (both uppercase and lowercase)
  let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  // Generate random characters
  for _ in 0..length {
    let random_index = rng.gen_range(0..alphabet.len());
    random_string.push(alphabet.chars().nth(random_index).unwrap());
  }

  random_string
}

impl IntoTag for Link {
  fn into_tag(&self) -> Vec<Tag> {
    let tag = Tag::Tag {
      classes: self.classes.clone(),
      ids: self.ids.clone(),
      ident: "a",
      children: Some(self.children.clone()),
      attributes: HashMap::from_iter([
        ("href".to_string(), self.href.to_string()),
        ("target".to_string(), self.target.to_string()),
      ]),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: if self.load_type == LinkLoadType::WhenIdle {
        HashSet::from_iter([self.href.clone()])
      } else {
        HashSet::default()
      },
    };

    let mut tags = Vec::from_iter([tag]);

    match self.load_type {
      LinkLoadType::No => tags,
      LinkLoadType::WhenIdle => tags,
      LinkLoadType::WhenHover => {
        let id: String = generate_random_letters(8);
        let script_tag = Tag::Tag {
          ident: "script",
          children: Some(Vec::from_iter([Tag::Text(format!(
            "/* Generated By route-rs */
document.querySelector(\"#{id}\").addEventListener('mouseenter', function() {{
  if (!window?.__ROUTE_RS_APPENDED_{id}) {{
     var link = document.createElement('link');
     link.rel = 'prefetch';
     link.href = '{}';
     document.head.appendChild(link);
   
     window.__ROUTE_RS_APPENDED_{id} = true;
   }}
}});",
            self.href
          ))])),
          ids: Vec::default(),
          classes: HashSet::default(),
          attributes: HashMap::default(),
          urls_to_preconnect: HashSet::default(),
          urls_to_prefetch: HashSet::default(),
        };

        tags[0].add_id(id);

        tags.push(script_tag);
        tags
      }
    }
  }
}

impl Link {
  pub fn new(href: String, children: Vec<Box<dyn IntoTag>>) -> Self {
    Self {
      href,
      children: children
        .into_iter()
        .flat_map(|x| x.into_tag().clone())
        .collect(),
      classes: HashSet::default(),
      ids: vec![],
      attributes: HashMap::default(),
      load_type: LinkLoadType::default(),
      target: LinkTarget::default(),
    }
  }

  pub fn text(href: impl Into<String>, text: &'static str) -> Self {
    Self::new(href.into(), vec![Box::new(text)])
  }

  pub fn preload(mut self, preload: LinkLoadType) -> Self {
    self.load_type = preload;
    self
  }
}
