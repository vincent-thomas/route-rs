pub fn wrap_lambda<H, Args>(handler: H) -> WrappedLambda<H, Args>
where
  H: titan_core::Handler<Args> + Sync + Clone,
  H::Future: std::future::Future<Output = H::Output> + Send,
  H::Output: titan_core::Respondable,
  Args: titan_core::FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  WrappedLambda(handler)
}

pub struct WrappedLambda<F, Args>(F)
where
  F: titan_core::Handler<Args>;

use futures_util::FutureExt;

impl<H, Args> titan_core::Service<LambdaRequest> for WrappedLambda<H, Args>
where
  H: titan_core::Handler<Args> + Sync + Clone,
  H::Future: std::future::Future<Output = H::Output> + Send,
  H::Output: titan_core::Respondable,
  Args: titan_core::FromRequest + Send + Sync + 'static,
  Args::Error: Send,
{
  type Response = H::Output;
  type Error = ();
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(
    &mut self,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: LambdaRequest) -> Self::Future {
    let req: Request = req.into();
    let args = match Args::from_request(req) {
      Ok(value) => value,
      Err(_) => panic!("oh no"),
    };

    Box::pin(self.0.call(args).map(|x| Ok(x)))
  }
}

impl From<LambdaRequest> for Request {
  fn from(value: LambdaRequest) -> Self {
    let (parts, body) = value.into_parts();

    let body = match body {
      lambda_http::Body::Text(text) => text.as_bytes().to_vec(),
      lambda_http::Body::Empty => Vec::default(),
      lambda_http::Body::Binary(binary) => binary,
    };

    Request::from_parts(parts, body.into())
  }
}
