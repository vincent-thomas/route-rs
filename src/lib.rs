use std::{future::Future, pin::Pin, sync::Arc};
pub mod address;

use address::Address;
use endpoint::Router;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::Service};
use matchit::Router;

use hyper_util::rt::{TokioIo, TokioTimer};
mod service;
pub use route_core::*;
pub use route_derive::*;
//pub mod types;
use tokio::net::TcpListener;
pub mod controller;
pub mod endpoint;
pub mod route;

pub use route_http as http;

#[derive(Clone, Default)]
pub struct App {
  inner: Arc<InnerApp>,
}

type RouteRouter = Router<EndpointRouter>;

#[derive(Default)]
struct InnerApp {
  routes: RouteRouter,
  bound_address: Option<Address>,
}

impl App {
  pub fn new() -> Self {
    App::default()
  }
}

pub struct Route<'a> {
  router: &'a mut RouteRouter,
  path: &'a str,
}

impl Route<'_> {
  fn method<F, Args>(&mut self, method: route_http::method::Method, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    let thing = self.router.at_mut(self.path);
    match thing {
      Ok(path) => {
        let thing = path.value;
        thing.method(method, handler);
        self
      }
      Err(_) => {
        let ep_router = EndpointRouter::default();
        let _ = self.router.insert(self.path, ep_router);
        self
      }
    }
  }

  pub fn get<F, Args>(&mut self, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    self.method(route_http::method::Method::GET, handler)
  }
  pub fn post<F, Args>(&mut self, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    self.method(route_http::method::Method::POST, handler)
  }
  pub fn put<F, Args>(&mut self, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    self.method(route_http::method::Method::PUT, handler)
  }
  pub fn patch<F, Args>(&mut self, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    self.method(route_http::method::Method::PATCH, handler)
  }
  pub fn delete<F, Args>(&mut self, handler: F) -> &mut Self
  where
    F: Endpoint<Args>,
    Args: FromRequest + 'static,
    F::Output: Respondable + 'static,
  {
    self.method(route_http::method::Method::DELETE, handler)
  }
}

impl App {
  pub fn at<'a>(&'a mut self, path: &'a str) -> Route<'a> {
    let ep_router = EndpointRouter::default();
    let thing = Arc::get_mut(&mut self.inner).unwrap();
    let _ = thing.routes.insert(path, ep_router);

    Route { router: &mut thing.routes, path }
  }
}

impl App {
  pub fn bind(mut self, address: Address) -> Self {
    let test = Arc::get_mut(&mut self.inner).unwrap();
    test.bound_address = Some(address);
    self
  }
  pub async fn listen(self, port: u16) {
    let address: String =
      self.inner.bound_address.clone().expect("address is required for listening").into();
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(host).await.unwrap();

    let app = Arc::new(self);

    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);

      let app = Arc::clone(&app);
      // let service = service::MainService::new(app_to_be_moved);
      tokio::task::spawn(async move {
        let mut http_client = http1::Builder::new();

        let result = http_client.timer(TokioTimer::new()).serve_connection(io, Nice { app }).await;

        if let Err(err) = result {
          eprintln!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}

struct Nice {
  app: Arc<App>,
}

impl Service<hyper::Request<hyper::body::Incoming>> for Nice {
  type Response = hyper::Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
    let app = Arc::clone(&self.app);
    Box::pin(async move {
      let endpoint_req = route_http::request::Request::new([].into());
      let uri = req.uri().clone();

      let route = match app.inner.routes.at(uri.path()) {
        Ok(route) => route,
        Err(_) => {
          return Ok(hyper::Response::new(Full::new(Bytes::from("404"))));
        }
      };

      let thing = route.value;

      let method = req.method();
      let fn_endpoint = thing.at(method);
      let endpoint_response = fn_endpoint.call(endpoint_req).await;
      let bytes = endpoint_response.clone().into_body();

      let mut response = hyper::Response::new(Full::new(Bytes::from_iter(bytes.to_vec())));

      *response.headers_mut() = endpoint_response.headers().clone();

      Ok(response)
    })
  }
}
