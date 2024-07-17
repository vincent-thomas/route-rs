use http::request::Parts;
pub use http::Request;

pub type HttpRequest = Request<Box<[u8]>>;

pub type Head = Parts;
