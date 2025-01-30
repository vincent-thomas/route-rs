#![forbid(unsafe_code)]

mod app;
mod build;
pub use build::build_static;
pub mod guard;
pub mod prelude;
pub mod route;
mod utils;
pub use titan_derive::ssg;

// For titan-derive
pub use utils::lazy_static;
pub use utils::FutureExt;

#[doc(hidden)]
pub mod lambda;
pub use app::*;
pub mod endpoint;

// Exported in web;
mod types;
pub mod web;

pub use titan_core::*;

#[doc(inline)]
pub use titan_html as html;
#[doc(inline)]
pub use titan_http as http;
pub use titan_server::*;
