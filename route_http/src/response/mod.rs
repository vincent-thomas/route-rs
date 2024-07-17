use http::response::Parts;

pub use http::response::Response;

pub type HttpResponse = Response<Box<[u8]>>;

pub type Head = Parts;
