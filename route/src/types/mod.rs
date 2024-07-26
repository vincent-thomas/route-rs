mod new_sse;
//use route_http::StatusCode;

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

pub struct Infallible;
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
