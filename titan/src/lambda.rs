use std::marker::PhantomData;
use std::{
  collections::HashMap, convert::Infallible, future::Future, pin::Pin,
};

use futures_util::FutureExt as _;
use lambda_http::{Request, Response};
use serde_json::Value;
use tower::Service;

use super::App;
use crate::always_ready;
use crate::handler::Handler;
use crate::http::{Extensions, FromRequest, Respondable};
use crate::AppFuture;

pub async fn run<'a, S>(service: S) -> Result<(), lambda_http::Error>
where
  S: Service<
    Request,
    Error = Infallible,
    Response = Response<lambda_http::Body>,
  >,
  S::Future: Future<Output = Result<S::Response, S::Error>> + Send + 'a,
{
  lambda_http::run(service).await
}

pub fn wrap_handler<H, Args>(handler: H) -> WrapHandler<H, Args>
where
  H: Handler<Args>,
  Args: FromRequest,
{
  WrapHandler(handler, PhantomData)
}

pub struct WrapHandler<H, Args>(H, PhantomData<Args>);

impl<H, Args> Service<Request> for WrapHandler<H, Args>
where
  H: Handler<Args>,
  H::Future: Future<Output = H::Output> + Send + 'static,
  H::Output: Respondable,
  Args: FromRequest + Send,
{
  type Response = Response<lambda_http::Body>;
  type Error = Infallible;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  always_ready!();

  fn call(&mut self, req: Request) -> Self::Future {
    let req = utils::from_lambda_request_titan(req);

    let args: Args = match FromRequest::from_request(req) {
      Ok(value) => value,
      Err(_) => unimplemented!(),
    };

    Box::pin(self.0.call(args).map(|output| {
      let res = output.respond();
      let lambda_res = utils::from_titan_response_lambda(res);
      Ok(lambda_res)
    }))
  }
}

impl Service<Request> for App {
  type Response = Response<lambda_http::Body>;
  type Error = Infallible;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  always_ready!();

  fn call(&mut self, req: Request) -> Self::Future {
    let uri = req.uri().clone();

    let mut req = utils::from_lambda_request_titan(req);

    match self.inner.router.lookup(uri.path()) {
      Some(endpoint) => {
        let params: HashMap<String, Value> =
          HashMap::from_iter(endpoint.params.iter().map(|(key, value)| {
            (key.to_string(), Value::from(value.to_string()))
          }));
        let mut extensions = Extensions::new();
        extensions.insert(params);

        *req.extensions_mut() = extensions;

        let mut service = endpoint.value.clone();

        Box::pin(AppFuture {
          fut: service.call(req).map(|x| match x {
            Ok(value) => Ok(utils::from_titan_response_lambda(value)),
            Err(_) => unreachable!(),
          }),
        })
      }
      None => {
        let mut fallback = self.inner.fallback.clone();
        Box::pin(AppFuture {
          fut: fallback.call(req).map(|x| match x {
            Ok(value) => Ok(utils::from_titan_response_lambda(value)),
            Err(_) => unreachable!(),
          }),
        })
      }
    }
  }
}

mod utils {
  use crate::http::Body;

  pub(crate) fn from_lambda_request_titan(
    req: lambda_http::http::Request<lambda_http::Body>,
  ) -> crate::http::Request {
    let (parts, body) = req.into_parts();

    let new_body = match body {
      lambda_http::Body::Text(text) => text.as_bytes().to_vec(),
      lambda_http::Body::Empty => Vec::default(),
      lambda_http::Body::Binary(bin) => bin,
    };

    let body = new_body.into_boxed_slice();
    let req = crate::http::Request::from_parts(parts, body);
    req
  }

  pub(crate) fn from_titan_response_lambda(
    res: crate::http::Response,
  ) -> lambda_http::Response<lambda_http::Body> {
    let (parts, body) = res.into_parts();

    let new_body = lambda_http::Body::Binary(match body {
      Body::Full(vec) => vec.to_vec(),
      Body::Stream(_) => {
        unimplemented!("stream not supported in lambda")
      }
    });
    lambda_http::Response::from_parts(parts, new_body)
  }
}
