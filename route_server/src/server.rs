use std::{net::SocketAddr, sync::Arc};

use http_body_util::Full;
use hyper::{
  body::{Bytes, Incoming},
  header::HeaderValue,
  server::conn::http1,
  service::Service,
  Response,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::findable::FindableRoute;

pub struct Server {
  socket: SocketAddr,
  #[allow(dead_code)]
  app: Arc<Box<dyn FindableRoute<'static>>>,
  inner: InnerServer,
}

impl Server {
  pub(crate) fn new(socket: SocketAddr, app: Box<dyn FindableRoute<'static>>) -> Self {
    Server { socket, app: Arc::new(app), inner: InnerServer }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(self.socket).await?;

    loop {
      let (stream, _) = listener.accept().await?;

      let io = TokioIo::new(stream);

      let inner_server = self.inner.clone();

      tokio::task::spawn(async move {
        if let Err(err) = http1::Builder::new().serve_connection(io, inner_server).await {
          eprintln!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}
#[allow(dead_code)]
fn into_hyper_response(resp: route_http::response::HttpResponse) -> hyper::Response<Bytes> {
  let mut res = hyper::Response::new(hyper::body::Bytes::new());
  *res.status_mut() = resp.status();
  for (key, value) in resp.headers() {
    res.headers_mut().append(key, value.clone());
  }
  *res.body_mut() = Bytes::from(resp.body().to_vec());
  res
}

#[allow(dead_code)]
fn into_hyper_request(req: hyper::Request<Incoming>) -> route_http::request::HttpRequest {
  let mut http_req = route_http::request::HttpRequest::new([].into());
  *http_req.headers_mut() = req.headers().clone();
  http_req
}

#[derive(Clone)]
struct InnerServer;

impl Service<hyper::Request<Incoming>> for InnerServer {
  type Response = hyper::Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
  >;

  #[allow(unused_variables)]
  fn call(&self, req: hyper::Request<Incoming>) -> Self::Future {
    Box::pin(async move {
      let mut response = Response::new(Full::new(Bytes::from_static("hello".as_bytes())));
      if cfg!(debug_assertions) {
        response.headers_mut().append("Server", HeaderValue::from_static("Route-RS"));
      }
      Ok(response)
    })
  }
}
