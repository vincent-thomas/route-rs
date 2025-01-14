use std::{
  collections::{HashMap, HashSet},
  hash::{Hash, Hasher},
};

use crate::{class::TagClass, context::Context};

pub mod head;
pub mod html;

mod image;
pub use image::*;

mod link;
pub use link::*;

mod script;
pub use script::*;

mod style;
pub use style::*;

pub trait IntoTag {
  fn into_tag(self) -> Tag;
}

impl IntoTag for Tag {
  fn into_tag(self) -> Tag {
    self
  }
}

impl Hash for Tag {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Tag::Text(text) => {
        // Hash the text contained in the Text variant
        text.hash(state);
      }
      Tag::Tag {
        ident,
        children,
        classes,
        ids,
        attributes,
        urls_to_preconnect,
        urls_to_prefetch,
      } => {
        ident.hash(state);

        match children {
          Some(children_list) => {
            children_list.hash(state);
          }
          None => {
            0u8.hash(state);
          }
        }

        for s in classes {
          s.hash(state);
        }

        ids.hash(state);

        for s in attributes {
          s.hash(state);
        }

        for s in urls_to_preconnect {
          s.hash(state);
        }

        for s in urls_to_prefetch {
          s.hash(state);
        }
      }
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
  Text(String),
  Tag {
    ident: &'static str,
    children: Option<Vec<Tag>>,
    classes: HashSet<TagClass>,
    ids: Vec<String>,

    attributes: HashMap<String, String>,

    urls_to_preconnect: HashSet<String>,
    urls_to_prefetch: HashSet<String>,
  },
}

impl Tag {
  pub fn children(&mut self) -> &mut Vec<Tag> {
    match self {
      Tag::Text(_) => unimplemented!(),
      Tag::Tag { children, .. } => {
        if let Some(value) = children {
          value
        } else {
          panic!("No children :(")
        }
      }
    }
  }
  pub fn add_id(&mut self, id: String) {
    if let Tag::Tag { ref mut ids, .. } = self {
      ids.push(id);
    }
  }

  pub(crate) fn apply_nonce(&mut self, nonce: &str) {
    match self {
      Tag::Text(_) => (),
      Tag::Tag { ident, ref mut attributes, ref mut children, .. } => {
        if *ident == "script" && children.is_some() {
          attributes.insert("nonce".to_string(), nonce.to_string());
        }
        if *ident == "style" {
          attributes.insert("nonce".to_string(), nonce.to_string());
        }

        if let Some(children) = children {
          for child in children.iter_mut() {
            child.apply_nonce(nonce);
          }
        }
      }
    }
  }

  pub(crate) fn hydrate(&mut self, ctx: &mut Context) {
    match self {
      Tag::Text(_) => (),
      Tag::Tag {
        ref mut classes,
        ref mut children,
        urls_to_preconnect,
        urls_to_prefetch,
        ..
      } => {
        {
          for url in urls_to_preconnect.iter() {
            ctx.add_preconnect(url.to_string());
          }

          for url in urls_to_prefetch.iter() {
            ctx.add_prefetch(url.to_string());
          }

          for class in classes.clone().into_iter() {
            if let TagClass::StyleRule(style) = class {
              ctx.add_styles(style.clone());
              classes.insert(TagClass::Normal(style.rule.to_string()));
              classes.remove(&TagClass::StyleRule(style));
            }
          }
        }

        if let Some(children) = children {
          for child in children.iter_mut() {
            child.hydrate(ctx);
          }
        }
      }
    };
  }
  pub(crate) fn to_string(&self) -> String {
    match self {
      Tag::Text(text) => text.clone(),
      Tag::Tag { ident, children, classes, ids, attributes, .. } => {
        let mut base = format!("<{ident}");

        if !attributes.is_empty() {
          let attributes_vec: Vec<String> = attributes
            .iter()
            .map(|x| format!("{key}=\"{value}\"", key = x.0, value = x.1))
            .collect();
          base.push_str(&format!(" {}", attributes_vec.join(" ")));
        };

        if !classes.is_empty() {
          let mut class_str = Vec::new();
          for class in classes {
            let class = match class {
              TagClass::Normal(class) => class.clone(),
              TagClass::StyleRule(style) => unreachable!("{:?}", style),
            };
            class_str.push(class);
          }
          base.push_str(format!(" class=\"{}\"", class_str.join(" ")).as_ref());
        }

        if !ids.is_empty() {
          let mut id_str = Vec::new();
          for id in ids {
            id_str.push(id.clone());
          }
          base.push_str(format!(" id=\"{}\"", id_str.join(" ")).as_ref());
        }
        base.push('>');

        if let Some(children) = children {
          let mut str_children = Vec::default();
          for child in children {
            let out = child.to_string();
            str_children.push(out);
          }
          let content = str_children.join("");
          base.push_str(&content);
          base.push_str(&format!("</{ident}>"));
        };

        base
      }
    }
  }
}

impl IntoTag for &'static str {
  fn into_tag(self) -> Tag {
    self.to_string().into_tag()
  }
}

impl IntoTag for String {
  fn into_tag(self) -> Tag {
    Tag::Text(self)
  }
}

macro_rules! impl_tag {
  ($($val:ident);*) => {
    $(
      titan_html_derive::html_tag! {
        #[derive(Debug, Default)]
        pub struct $val {
          pub children: Vec<Tag>,
        }
      }

      impl IntoTag for $val {
        fn into_tag(self) -> Tag {
          Tag::Tag {
              ident: paste::paste! { stringify!([<$val:lower>]) },
              children: Some(self.children),
              classes: self.classes,
              ids: self.ids,
              attributes: HashMap::default(),

              urls_to_preconnect: HashSet::default(),
              urls_to_prefetch: HashSet::default()
          }
        }
      }

      impl $val {
        pub fn text(value: impl Into<String>) -> Self {
           $val {
               children: Vec::from_iter([Tag::Text(value.into())]),
               classes: HashSet::default(),
               ids: Vec::default(),
               attributes: HashMap::default(),
           }
        }

        pub fn children<I>(mut self, children: I) -> Self where I: IntoIterator<Item = Tag> {
           self.children = children.into_iter().collect::<Vec<Tag>>();
           self
        }
      }
    )*
  };
}

impl_tag! {
    Body;

    // Content Section
    Header;
    Nav;
    Main;
    Footer;
    Articles;
    Aside;
    Section;

    // Text Content
    BlockQuote;
    Div;
    Dl;
    Dt;
    Hr;
    Li;
    Ol;
    P;
    Pre;
    Ul;
    // A is a special one. Use [Link] instead
    B;
    Bdi;
    Bdo;
    Br;
    Cite;
    Code;
    Data;
    Em;
    I;
    Kbd;
    Mark;
    Q;
    S;
    Small;
    Span;
    Strong;
    Sub;
    Sup;
    Time;
    U;
    Var;
    Wbr;

    // Scripting
    NoScript;
    // TODO: Script

    H1;
    H2;
    H3;
    H4;
    H5;
    H6
}
