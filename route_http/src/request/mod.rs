use http::request::Parts;
pub use http::Request;
use std::str::FromStr;
#[cfg(feature = "parser")]
mod parser_from_str;
#[cfg(feature = "parser")]
pub use parser_from_str::*;

pub type HttpRequest = Request<Box<[u8]>>;
pub type Head = Parts;
