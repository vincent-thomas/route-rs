mod json;
mod urlencoded;

pub use json::*;
pub use urlencoded::*;

pub enum BodyParseError {
  ContentTypeInvalid,
  NoBody,
}
