pub mod bytes;
pub mod json;
pub mod redirect;
pub mod urlencoded;

pub enum BodyParseError {
  ContentTypeInvalid,
  NoBody,
}
