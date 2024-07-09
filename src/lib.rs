use std::sync::Arc;
mod handler;

use handler::Handler;
use route_http::{method::HttpMethod, response::Respondable, HttpRequest};
use route_router::{Route, Router};

use hyper_util::{
  rt::{TokioExecutor, TokioIo},
  server::conn::auto,
};
mod service;
pub use route_derive::*;
use tokio::net::TcpListener;

pub use route_http as http;

pub struct App<Address>
where
  Address: Clone,
{
  routes: Router<Box<dyn Handler>>,
  bound_address: Address,
}

#[derive(Clone)]
pub struct NoBoundAddress;
#[derive(Clone)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl From<Address> for String {
  fn from(val: Address) -> Self {
    format!("{}.{}.{}.{}", val.0, val.1, val.2, val.3)
  }
}

pub trait Service {
  fn method(&self) -> HttpMethod;
  fn path(&self) -> String;
  fn handler(self, req: HttpRequest) -> impl Respondable;
}

impl Default for App<NoBoundAddress> {
  fn default() -> Self {
    App { routes: Router::mount_root(), bound_address: NoBoundAddress }
  }
}

impl App<NoBoundAddress> {
  pub fn mount_at(path: String) -> Self {
    App { routes: Router::mount_at(path), bound_address: NoBoundAddress }
  }

  pub fn service<S, O, F, H>(&mut self, route_service: S, handler: H)
  where
    H: Handler + Clone,
    S: Service + 'static,
    O: 'static + Respondable + Send,
    F: 'static + std::future::Future<Output = O>,
  {
    let method = route_service.method();
    let path = route_service.path();
    let route = Route::new(handler);
    self.routes.route(method, path, route);
  }
  pub fn bind(self, address: Address) -> App<Address> {
    App { routes: self.routes, bound_address: address }
  }
}

impl App<Address> {
  pub async fn listen(self, port: u16) {
    let address: String = self.bound_address.clone().into();
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(host).await.unwrap();

    let app = Arc::new(self);

    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);

      let app_to_be_moved = Arc::clone(&app);

      tokio::task::spawn(async move {
        let http_client = auto::Builder::new(TokioExecutor::new());
        let service = service::MainService::new(app_to_be_moved);

        let result = http_client.serve_connection(io, service).await;

        if let Err(err) = result {
          eprintln!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}
