mod lambda_service;
use std::future::Future;

pub type Request = lamda_http::Request;

use lambda_service::LambdaHandlerService;
use titan_core::{FromRequest, Handler, Respondable};

pub fn run() {
  lambda_http::run(self).await
}

//pub fn handler_runtime<H, Args>(handler: H) -> LambdaHandlerService<H, Args>
//where
//  H: Handler<Args> + Clone,
//  H::Future: Future<Output = H::Output> + Send,
//  H::Output: Respondable,
//  Args: FromRequest + Send + Sync + 'static,
//  Args::Error: Send,
//{
//  LambdaHandlerService::new(handler)
//}

//use futures_util::{FutureExt, StreamExt as _};
//use lambda_http::lambda_runtime::Diagnostic;
//use titan_core::{FromRequest, Handler, Respondable, Service};

//pub fn wrap_lambda<H, Args>(handler: H) -> LambdaService<H, Args>
//where
//  H: Handler<Args> + Clone,
//  H::Future: Future<Output = H::Output> + Send,
//  H::Output: Respondable,
//  Args: FromRequest + Send + Sync + 'static,
//  Args::Error: Send,
//{
//  LambdaService::new(handler)
//}
