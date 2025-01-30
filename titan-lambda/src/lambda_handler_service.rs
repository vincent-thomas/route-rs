use std::{
  future::Future,
  marker::PhantomData,
  pin::Pin,
  task::{Context, Poll},
};

use futures_util::FutureExt as _;
use lambda_http::{
  lambda_runtime::Diagnostic, Body as LambdaBody, Request as LambdaRequest,
};

use titan_core::{FromRequest, Handler, Respondable, Service};
use titan_http::{
  body::Body as TitanBody, Request as TitanRequest, Response as TitanResponse,
};

pub struct LambdaHandlerService<H, Args> {
  f: H,
  _args: PhantomData<Args>,
}

impl<H, Args> From<H> for LambdaHandlerService<H, Args>
where
  H: Handler<Args>,
{
  fn from(value: H) -> Self {
    Self { f: value, _args: PhantomData }
  }
}

#[derive(Debug)]
pub struct LambdaError;

impl From<LambdaError> for Diagnostic {
  fn from(_: LambdaError) -> Diagnostic {
    Diagnostic {
      error_type: "strange".into(),
      error_message: "this shouldn't happen".into(),
    }
  }
}

impl<H, Args> Service<LambdaRequest> for LambdaHandlerService<H, Args>
where
  Args: FromRequest,
  H: Handler<Args>,
  H::Future: Future<Output = H::Output> + Send + 'static,
  H::Output: Respondable,
{
  type Response = TitanResponse<lambda_http::Body>;
  type Error = LambdaError;

  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(
    &mut self,
    _: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: LambdaRequest) -> Self::Future {
    let (parts, body) = req.into_parts();

    let body = match body {
      LambdaBody::Text(text) => text.as_bytes().to_vec(),
      LambdaBody::Empty => Vec::default(),
      LambdaBody::Binary(bin) => bin,
    }
    .into_boxed_slice();

    let req = TitanRequest::from_parts(parts, body);

    let args = match Args::from_request(req) {
      Ok(value) => value,
      Err(_) => return Box::pin(async move { Err(LambdaError) }),
    };
    let fut = self.f.call(args).map(|x| {
      let body = x.respond();

      let (parts, body) = body.into_parts();
      let new_body = match body {
        TitanBody::Full(full) => lambda_http::Body::Binary(full.to_vec()),
        TitanBody::Stream(_) => panic!("bnono"),
      };
      let res = TitanResponse::from_parts(parts, new_body);
      Ok(res)
    });
    Box::pin(fut)
  }
}
