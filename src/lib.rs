use std::convert::Infallible;
pub mod address;
pub mod endpoint;
pub mod handler;

use address::Address;
use handler::Handler;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http2::Builder, service::service_fn};
use matchit::Router;

use hyper_util::rt::{TokioExecutor, TokioIo};
mod service;
pub use route_derive::*;
use tokio::net::TcpListener;

pub use route_http as http;

#[derive(Clone)]
pub struct App<T = ()> {
  routes: Router<T>,
  bound_address: Option<Address>,
}

impl<T> Default for App<T> {
  fn default() -> App<T> {
    App { routes: Router::new(), bound_address: None }
  }
}

impl<T> App<T>
where
  T: Handler,
{
  pub fn service(&mut self, path: &str, route_service: T) {
    let _ = self.routes.insert(path, route_service);
  }
  pub fn bind(self, address: Address) -> App<T> {
    App { routes: self.routes, bound_address: Some(address) }
  }
}

impl<T> App<T>
where
  T: Send + Sync,
{
  pub async fn listen(self, port: u16) {
    let address: String = self.bound_address.expect("address is required for listening").into();
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(host).await.unwrap();

    // let app = Arc::new(self.routes);

    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);

      // let app_to_be_moved = Arc::clone(&app);

      // let service = service::MainService::new(app_to_be_moved);
      tokio::task::spawn(async move {
        let http_client = Builder::new(TokioExecutor::new());

        let result = http_client.serve_connection(io, service_fn(nice_service)).await;

        if let Err(err) = result {
          eprintln!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}

async fn nice_service(
  _req: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
  let mut res = hyper::Response::new(Full::new(Bytes::from("test")));
  *res.status_mut() = hyper::StatusCode::OK;
  Ok(res)
}
