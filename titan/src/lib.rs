mod build;
pub mod html;
mod router;
use std::future::Future;
use std::pin::Pin;

pub use build::build_static;
pub use router::Router;

mod utils;

mod serve;
#[doc(inline)]
pub use serve::serve;

pub mod guard;
pub mod handler;
pub mod http;
pub mod prelude;
pub mod route;

#[cfg(feature = "lambda")]
pub mod lambda;

mod app;
#[doc(inline)]
pub use app::*;

pub mod endpoint;

// Exported in web;
mod types;
pub mod web;

mod macros;

// For titan-derive
#[doc(inline)]
pub use titan_derive::ssg;
pub use utils::lazy_static;
pub use utils::FutureExt;

pub use tower::Service;

pub(crate) type BoxedSendFuture<O> = Pin<Box<dyn Future<Output = O> + Send>>;
