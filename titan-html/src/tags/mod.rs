use std::{
  collections::{HashMap, HashSet},
  hash::{Hash, Hasher},
};

use crate::{class::TagClass, context::Context, stylerule::StyleRule};
use cssparser::{Parser, ParserInput};
use lightningcss::{
  declaration::DeclarationBlock, printer::PrinterOptions,
  stylesheet::ParserOptions,
};
pub mod head;
pub mod html;
pub mod image;
pub mod link;
pub mod style;

pub trait IntoTag: Clone {
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

          for class in classes.clone().iter() {
            if let TagClass::Style(style) = class {
              let mut parser_input = ParserInput::new(style);
              let mut parser = Parser::new(&mut parser_input);
              let parsed_styles =
                DeclarationBlock::parse(&mut parser, &ParserOptions::default())
                  .expect("Invalid css");

              for style in parsed_styles.declarations {
                let options =
                  PrinterOptions { minify: true, ..Default::default() };
                let id = style.property_id();
                let key = id.name().to_string();
                let value = style.value_to_css_string(options).unwrap();

                let rule = StyleRule::from_iter([(key, value)]);
                ctx.add_styles(rule.clone());
                classes.insert(TagClass::Normal(rule.rule));
              }
              for style in parsed_styles.important_declarations {
                let options =
                  PrinterOptions { minify: true, ..Default::default() };
                let id = style.property_id();
                let key = id.name().to_string();
                let value = style.value_to_css_string(options).unwrap();

                let rule = StyleRule::from_iter([(key, value)]);
                ctx.add_styles(rule.clone());
                classes.insert(TagClass::Normal(rule.rule));
              }

              classes.remove(&TagClass::Style(style.clone()));
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
              TagClass::Style(style) => unreachable!("{:?}", style),
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

impl From<String> for Tag {
  fn from(value: String) -> Self {
    Self::Text(value)
  }
}

impl IntoTag for &'static str {
  fn into_tag(self) -> Tag {
    Tag::Text(self.to_string())
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
    )*
  };
}
impl_tag! { Div; Body; Span; P; Header; Footer }

macro_rules! impl_children_tag {
  ($($val:ident);*) => {
    $(
        impl $val {
          pub fn text(value: &'static str) -> $val {
             $val {
                 children: Vec::from_iter([Tag::Text(value.to_string())]),
                 classes: HashSet::default(),
                 ids: Vec::default(),
                 attributes: HashMap::default(),
             }
          }
        }

        impl<I> From<I> for $val where I: IntoIterator<Item = Tag> {
          fn from(value: I) -> $val {
              $val {
                  children: value.into_iter().collect::<Vec<Tag>>(),
                  classes: HashSet::default(),
                  ids: Vec::default(),
                  attributes: HashMap::default(),
              }
          }
        }
    )*
  };
}

impl_children_tag! { Div; P; Span; Header; Body }
