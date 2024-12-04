pub use http::request::Parts;
mod parser_from_str;
pub use parser_from_str::*;

pub type Request = http::Request<Box<[u8]>>;
