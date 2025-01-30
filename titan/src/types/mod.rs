use crate::http::{
  header::HeaderName, response::Builder, Body, Respondable, Response,
};

macro_rules! host_use {
  ($($name:ident),*) => {
    $(
      mod $name;
      pub use self::$name::*;
    )*
  };
}

host_use! {
  bytes,
  json,
  redirect,
  urlencoded,
  cookie,
  query,
  params,
  sse,
  path
}

pub mod authorization;

pub enum BodyParsingError {
  NoBody,
  ContentTypeInvalid,
  InvalidBody,
  ParsingError(String),
}

impl Respondable for BodyParsingError {
  fn respond(self) -> Response {
    let body = match self {
      Self::NoBody => "Body is empty".into(),
      Self::InvalidBody => "Invalid Body".into(),
      Self::ContentTypeInvalid => "Invalid content-type".into(),
      Self::ParsingError(err) => format!("Body Parsing Error: {}", err),
    };

    Builder::default()
      .status(400)
      .header(HeaderName::from_static("content-type"), "text/plain")
      .header(HeaderName::from_static("content-length"), body.len().to_string())
      .body(Body::from(body))
      .unwrap()
  }
}
