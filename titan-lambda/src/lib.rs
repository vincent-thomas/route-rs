mod lambda_service;
use std::future::Future;

pub use lambda_http::Request;

use lambda_service::LambdaHandlerService;
use titan_core::{FromRequest, Handler, Respondable};

pub fn handler_runtime<H, Args>(handler: H) -> LambdaHandlerService<H, Args>
where
  H: Handler<Args> + Clone,
  H::Future: Future<Output = H::Output> + Send,
  H::Output: Respondable,
  Args: FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  LambdaHandlerService::new(handler)
}
