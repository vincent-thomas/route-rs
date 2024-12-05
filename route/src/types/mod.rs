//mod new_sse;
//use route_http::StatusCode;

pub mod authorization;

use route_core::Respondable;
use route_http::{body::Body, header::HeaderName, response::Response};

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
  sse,
  long_polling
}

pub enum BodyParsingError {
  NoBody,
  ContentTypeInvalid,
  InvalidBody,
  ParsingError(String),
}

impl Respondable for BodyParsingError {
  fn respond(self) -> route_http::response::Response<route_http::body::Body> {
    let body = match self {
      Self::NoBody => "Body is empty".into(),
      Self::InvalidBody => "Invalid Body".into(),
      Self::ContentTypeInvalid => "Invalid body type".into(),
      Self::ParsingError(err) => format!("Parsing error: {}", err),
    };

    Response::builder()
      .status(400)
      .header(HeaderName::from_static("content-type"), "text/plain")
      .header(HeaderName::from_static("content-length"), body.len().to_string())
      .body(Body::from(body))
      .unwrap()
  }
}

//
// #[derive(Debug)]
// pub enum BodyParseError {
//   ContentTypeInvalid,
//   NoBody,
// }
//
// impl Into<Error> for BodyParseError {
//   fn into(self) -> Error {
//     match self {
//       BodyParseError::ContentTypeInvalid => Error::new(
//         StatusCode::BAD_REQUEST,
//         b"Bad Request".to_vec().into_boxed_slice(),
//       ),
//       BodyParseError::NoBody => Error::new(
//         StatusCode::BAD_REQUEST,
//         b"Request body is empty".to_vec().into_boxed_slice(),
//       ),
//     }
//   }
// }
//
// impl Into<Error> for Infallible {
//   fn into(self) -> Error {
//     Error::new(
//       StatusCode::INTERNAL_SERVER_ERROR,
//       b"Internal Server Error".to_vec().into_boxed_slice(),
//     )
//   }
// }
