use titan_core::Respondable;
use titan_http::{body::Body, header::HeaderName, response::Response};

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
  sse
}

pub mod authorization;

pub enum BodyParsingError {
  NoBody,
  ContentTypeInvalid,
  InvalidBody,
  ParsingError(String),
}

impl Respondable for BodyParsingError {
  fn respond(self) -> titan_http::response::Response<titan_http::body::Body> {
    let body = match self {
      Self::NoBody => "Body is empty".into(),
      Self::InvalidBody => "Invalid Body".into(),
      Self::ContentTypeInvalid => "Invalid content-type".into(),
      Self::ParsingError(err) => format!("Body Parsing Error: {}", err),
    };

    Response::builder()
      .status(400)
      .header(HeaderName::from_static("content-type"), "text/plain")
      .header(HeaderName::from_static("content-length"), body.len().to_string())
      .body(Body::from(body))
      .unwrap()
  }
}
