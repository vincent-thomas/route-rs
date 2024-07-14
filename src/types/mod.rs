mod bytes;
pub use bytes::*;
mod json;
pub use json::*;
mod redirect;
pub use redirect::*;
mod urlencoded;
pub use urlencoded::*;

pub enum BodyParseError {
  ContentTypeInvalid,
  NoBody,
}
