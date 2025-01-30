#![forbid(unsafe_code)]
mod router;
pub mod segment;
mod segments;
pub use router::*;
pub use titan_router_derive::define_routes;
