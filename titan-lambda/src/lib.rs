use std::{convert::Infallible, future::Future, pin::Pin, task::Poll};

use lambda_http::Request as LambdaRequest;
use titan_core::FromRequest;
use titan_http::request::Request;

//impl FromRequest for LambdaRequest {
//  type Error = Infallible;
//
//  fn from_request(req: Request) -> Result<Self, Self::Error> {
//
//  }
//}

//S: Service<Request, Response = R, Error = E>,
//   S::Future: Send + 'a,
//   R: IntoResponse,
//   E: std::fmt::Debug + Into<Diagnostic>,
