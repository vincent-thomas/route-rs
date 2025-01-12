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

// TODO: quick fetch
// function quicklink(path) {
//    let doc = await fetch(path).then(v => v.text())
//      let parsed = new DOMParser().parseFromString(nice, "text/html");
//      document.querySelector("body").replaceWith(parsed.querySelector("body"))
//      document.querySelector("head").replaceWith(parsed.querySelector("head"))
// }
titan_html_derive::html_tag! {
  pub struct Link {
    pub children: Vec<Tag>,
    href: String,
    target: LinkTarget,
    pub load_type: LinkLoadType,
  }
}

//use rand::Rng as _;
//fn generate_random_letters(length: usize) -> String {
//  let mut rng = rand::thread_rng();
//  let mut random_string = String::new();
//
//  // Define the alphabet (both uppercase and lowercase)
//  let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
//
//  // Generate random characters
//  for _ in 0..length {
//    let random_index = rng.gen_range(0..alphabet.len());
//    random_string.push(alphabet.chars().nth(random_index).unwrap());
//  }
//
//  random_string
//}

impl IntoTag for Link {
  fn into_tag(self) -> Tag {
    let mut tag = Tag::Tag {
      classes: self.classes.clone(),
      ids: self.ids.clone(),
      ident: "a",
      children: Some(self.children.clone()),
      attributes: HashMap::from_iter([
        ("href".to_string(), self.href.to_string()),
        ("target".to_string(), self.target.to_string()),
      ]),
      urls_to_preconnect: HashSet::default(),
      urls_to_prefetch: HashSet::default(),
    };

    match self.load_type {
      LinkLoadType::No => tag,
      LinkLoadType::WhenIdle => {
        if let Tag::Tag { ref mut urls_to_prefetch, .. } = &mut tag {
          *urls_to_prefetch = HashSet::from_iter([self.href.clone()]);
        };
        tag
      }
      LinkLoadType::WhenHover => {
        unimplemented!()
        //        let id: String = generate_random_letters(8);
        //        let script_tag = Tag::Tag {
        //          ident: "script",
        //          children: Some(Vec::from_iter([Tag::Text(format!(
        //            "/* Generated By route-rs */
        //document.querySelector(\"#{id}\").addEventListener('mouseenter', function() {{
        //  if (!window?.__TITAN_RS_APPENDED_{id}) {{
        //     var link = document.createElement('link');
        //     link.rel = 'prefetch';
        //     link.href = '{}';
        //     document.head.appendChild(link);
        //
        //     window.__TITAN_RS_APPENDED_{id} = true;
        //   }}
        //}});",
        //            self.href
        //          ))])),
        //          ids: Vec::default(),
        //          classes: HashSet::default(),
        //          attributes: HashMap::default(),
        //          urls_to_preconnect: HashSet::default(),
        //          urls_to_prefetch: HashSet::default(),
        //        };
        //
        //        tags[0].add_id(id);
        //
        //        tags.push(script_tag);
        //        tags
      }
    }
  }
}

impl Link {
  pub fn new<C>(href: String, children: C) -> Self
  where
    C: IntoIterator<Item = Tag>,
  {
    let children: Vec<Tag> = children.into_iter().collect();
    Self {
      href,
      children,
      classes: HashSet::default(),
      ids: vec![],
      attributes: HashMap::default(),
      load_type: LinkLoadType::default(),
      target: LinkTarget::default(),
    }
  }

  pub fn text(href: impl Into<String>, text: &'static str) -> Self {
    Self::new(href.into(), vec![text.to_string().into_tag()])
  }

  pub const fn preload(mut self, preload: LinkLoadType) -> Self {
    self.load_type = preload;
    self
  }
}
