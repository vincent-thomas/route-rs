use http::request::Parts;
pub use http::Request;
mod parser_from_str;
pub use parser_from_str::*;

pub type HttpRequest = Request<Box<[u8]>>;
pub type Head = Parts;
