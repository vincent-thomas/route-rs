use std::{
  collections::HashMap, convert::Infallible, future::Future, pin::Pin,
};

use futures_util::FutureExt as _;
use lambda_http::{Request, Response};
use serde_json::Value;
use titan_core::Service;

use crate::AppFuture;

use super::App;
#[derive(Debug)]
pub struct LambdaError;

//impl From<LambdaError> for Diagnostic {
//  fn from(_: LambdaError) -> Diagnostic {
//    Diagnostic {
//      error_type: "strange".into(),
//      error_message: "this shouldn't happen".into(),
//    }
//  }
//}

pub struct LambdaAppService {
  app: App,
}

impl LambdaAppService {
  pub fn new(app: App) -> Self {
    LambdaAppService { app }
  }

  pub async fn run(self) -> Result<(), lambda_http::Error> {
    lambda_http::run(self.app).await
  }
}

pub fn from_lambda_request_titan(
  req: lambda_http::http::Request<lambda_http::Body>,
) -> titan_http::Request {
  let (parts, body) = req.into_parts();

  let new_body = match body {
    lambda_http::Body::Text(text) => text.as_bytes().to_vec(),
    lambda_http::Body::Empty => Vec::default(),
    lambda_http::Body::Binary(bin) => bin,
  };

  let body = new_body.into_boxed_slice();
  let req = titan_http::Request::from_parts(parts, body);
  req
}

pub fn from_titan_response_lambda(
  req: titan_http::Response,
) -> lambda_http::Response<lambda_http::Body> {
  let (parts, body) = req.into_parts();

  let new_body = lambda_http::Body::Binary(match body {
    titan_http::body::Body::Full(vec) => vec.to_vec(),
    titan_http::body::Body::Stream(stream) => {
      panic!("stream not supported in lambda")
    }
  });
  Response::from_parts(parts, new_body)
}

impl Service<Request> for App {
  type Response = Response<lambda_http::Body>;
  type Error = Infallible;
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, mut req: Request) -> Self::Future {
    let uri = req.uri().clone();

    let mut req = from_lambda_request_titan(req);

    match self.inner.router.lookup(uri.path()) {
      Some(endpoint) => {
        let params: HashMap<String, Value> =
          HashMap::from_iter(endpoint.params.iter().map(|(key, value)| {
            (key.to_string(), Value::from(value.to_string()))
          }));
        let mut extensions = titan_http::Extensions::new();
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
