use std::collections::HashMap;

use crate::{class::TagClass, head::Head, style::Style};

#[derive(Debug, Default)]
pub struct Html {
  pub head: Head,
  pub body: Body,
}
impl Html {
  pub fn with_head(head: Head) -> Self {
    Html { head, body: Body::default() }
  }
  pub fn body_from_iter<T, In>(&mut self, tags: T)
  where
    T: IntoIterator<Item = In>,
    In: IntoTag,
  {
    let tags: Vec<Tag> = tags.into_iter().flat_map(|x| x.into_tag()).collect();
    self.body.children.extend(tags);
  }
}

impl IntoTag for Html {
  fn into_tag(&self) -> Vec<Tag> {
    let mut children = Vec::default();

    children.extend(self.head.into_tag());
    children.extend(self.body.into_tag());

    Vec::from_iter([
      Tag::Text("<!DOCTYPE html>".to_string()),
      Tag::Tag {
        children: Some(children),
        classes: Vec::default(),
        attributes: HashMap::default(),
        ident: "html",
        ids: Vec::default(),
      },
    ])
  }
}
pub trait IntoTag {
  fn into_tag(&self) -> Vec<Tag>;
}

impl IntoTag for Tag {
  fn into_tag(&self) -> Vec<Tag> {
    vec![self.clone()]
  }
}

#[derive(Clone, Debug)]
pub enum Tag {
  Text(String),
  Tag {
    ident: &'static str,
    children: Option<Vec<Tag>>,
    classes: Vec<TagClass>,
    ids: Vec<String>,

    attributes: HashMap<String, String>,
  },
}

impl Tag {
  pub fn add_id(&mut self, id: String) {
    if let Tag::Tag { ref mut ids, .. } = self {
      ids.push(id);
    }
  }
  pub(crate) fn hydrate_styles(&self, styles: &mut Style) {
    match self {
      Tag::Text(_) => (),
      Tag::Tag { classes, children, .. } => {
        if !classes.is_empty() {
          for class in classes {
            if let TagClass::Style(style) = class {
              styles.add_rule(style.clone());
            };
          }
        }

        if let Some(children) = children {
          for child in children {
            child.hydrate_styles(styles)
          }
        }
      }
    }
  }
  pub(crate) fn to_string(&self) -> String {
    match self {
      Tag::Text(text) => text.clone(),
      Tag::Tag { ident, children, classes, ids, attributes } => {
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
              TagClass::Style(style) => style.rule.clone(),
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
  fn into_tag(&self) -> Vec<Tag> {
    Vec::from_iter([Tag::Text(self.to_string())])
  }
}

macro_rules! testing {
  ($($val:ident $char:ident);*) => {
    $(

        #[derive(Debug, Default)]
        pub struct $val {
          pub children: Vec<Tag>,
          pub classes: Vec<TagClass>,
          pub ids: Vec<String>,
        }

            impl IntoTag for $val {
              fn into_tag(&self) -> Vec<Tag> {
                Vec::from_iter([Tag::Tag {
                    ident: stringify!($char),
                    children: Some(self.children.clone()),
                    classes: self.classes.clone(),
                    ids: self.ids.clone(),
                    attributes: HashMap::default()
                }])
            }
        }
    )*
  };
}
testing! { Div div; Body body; Span span }
