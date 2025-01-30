use std::{
  collections::HashMap, convert::Infallible, future::Future, pin::Pin,
};

use futures_util::FutureExt as _;
use lambda_http::{Request, Response};
use serde_json::Value;
use tower::Service;

use crate::{always_ready, macros};

use crate::http::{Body, Extensions};

use crate::AppFuture;

use super::App;
#[derive(Debug)]
pub struct LambdaError;

pub struct LambdaService<S> {
  service: S,
}

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

pub fn from_lambda_request_titan(
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

pub fn from_titan_response_lambda(
  req: crate::http::Response,
) -> lambda_http::Response<lambda_http::Body> {
  let (parts, body) = req.into_parts();

  let new_body = lambda_http::Body::Binary(match body {
    Body::Full(vec) => vec.to_vec(),
    Body::Stream(_) => {
      unimplemented!("stream not supported in lambda")
    }
  });
  Response::from_parts(parts, new_body)
}

impl Service<Request> for App {
  type Response = Response<lambda_http::Body>;
  type Error = Infallible;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  always_ready!();

  fn call(&mut self, req: Request) -> Self::Future {
    let uri = req.uri().clone();

    let mut req = from_lambda_request_titan(req);

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
            Ok(value) => Ok(from_titan_response_lambda(value)),
            Err(_) => unreachable!(),
          }),
        })
      }
      None => {
        let mut fallback = self.inner.fallback.clone();
        Box::pin(AppFuture {
          fut: fallback.call(req).map(|x| match x {
            Ok(value) => Ok(from_titan_response_lambda(value)),
            Err(_) => unreachable!(),
          }),
        })
      }
    }
  }
}
