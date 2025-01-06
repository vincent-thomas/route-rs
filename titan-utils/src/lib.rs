use std::{error::Error, future::Future, pin::Pin};

#[cfg(feature = "internal-cssvalidate")]
pub mod validatecss;

pub type BoxStdError = Box<dyn Error>;

pub type BoxedFuture<O> = Pin<Box<dyn Future<Output = O>>>;
pub type BoxedSendFuture<O> = Pin<Box<dyn Future<Output = O> + Send>>;
