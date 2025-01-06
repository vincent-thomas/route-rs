use crate::prelude::*;
use route_core::Service;
use route_http::{
  body::Body,
  header::{HeaderValue, CONTENT_LENGTH},
  request::{HttpRequestExt, Request},
  response::{Response, ResponseBuilder},
};
use route_router::Router;
use route_server::IncomingStream;
use route_utils::BoxedSendFuture;
use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tokio::{
  io::{AsyncBufReadExt as _, AsyncReadExt as _, BufReader},
  net::TcpStream,
};

#[derive(Clone)]
pub struct App {
  inner: Arc<AppInner>,
}

impl Default for App {
  fn default() -> Self {
    Self { inner: Arc::new(AppInner(Router::default())) }
  }
}

struct AppInner(Router<BoxCloneService<Request, Response, Response>>);

macro_rules! tap_inner {
    ( $self_:ident, mut $inner:ident => { $($stmt:stmt)* } ) => {
        #[allow(redundant_semicolons)]
        {
            let mut $inner = $self_.into_inner();
            $($stmt)*
            App {
                inner: Arc::new($inner),
            }
        }
    };
}

impl App {
  fn into_inner(self) -> AppInner {
    match Arc::try_unwrap(self.inner) {
      Ok(inner) => inner,
      Err(arc) => AppInner(arc.0.clone()),
    }
  }
  pub fn at<S>(self, path: &str, endpoint: S) -> Self
  where
    S: Service<
        Request,
        Response = Response,
        Error = Response,
        Future = BoxedSendFuture<Result<Response, Response>>,
      >
      + 'static
      + Clone
      + Sync
      + Send,
  {
    tap_inner!(self, mut this => {
      this.0.at(path, BoxCloneService::new(endpoint));
    })
  }
}

impl<'a> Service<IncomingStream<'a>> for App {
  type Response = Response<Body>;
  type Error = Response<Body>;
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, incoming: IncomingStream<'a>) -> Self::Future {
    Box::pin(async move {
      let mut buf_reader = BufReader::new(incoming.0);
      let http_headers = read_request(&mut buf_reader).await.join("\n");

      let req_empty_body = HttpRequestExt::from(http_headers).0;
      let body_length = req_empty_body
        .headers()
        .get(CONTENT_LENGTH)
        .unwrap_or(&HeaderValue::from(0))
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
      let mut req =
        fill_req_body(req_empty_body, body_length, buf_reader).await;
      let uri = req.uri().clone();
      match self.inner.0.lookup(uri.path()) {
        Some(endpoint) => {
          let params: HashMap<String, Value> =
            HashMap::from_iter(endpoint.params.iter().map(|(key, value)| {
              (key.to_string(), Value::from(value.to_string()))
            }));
          let mut extensions = route_http::Extensions::new();
          extensions.insert(params);

          *req.extensions_mut() = extensions;

          let mut service = endpoint.value.clone();

          service.call(req).await
        }
        None => {
          let response =
            ResponseBuilder::new().status(404).body(Body::from(())).unwrap();
          Ok(response)
        }
      }
    })
  }
}

pin_project_lite::pin_project! {
  struct AppFuture<F> {
      #[pin]
      fut: F
  }
}

impl<F> Future for AppFuture<F>
where
  F: Future,
{
  type Output = F::Output;

  fn poll(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let this = self.project();

    this.fut.poll(cx)
  }
}

pub async fn read_request(
  reader: &mut BufReader<&mut TcpStream>,
) -> Vec<String> {
  let mut request_lines = Vec::new();
  loop {
    let mut line = String::new();
    if let Ok(0) = reader.read_line(&mut line).await {
      // End of buffer if 0 bytes left.
      break;
    }
    if line.trim().is_empty() {
      break;
    }
    request_lines.push(line.trim().to_string());
  }
  request_lines
}

pub async fn fill_req_body(
  mut req: Request,
  body_length: usize,
  mut reader: BufReader<&mut TcpStream>,
) -> Request {
  if body_length == 0 {
    return req;
  };
  let mut body = vec![0u8; body_length];
  reader.read_exact(&mut body).await.unwrap();
  *req.body_mut() = body.into();
  req
}
