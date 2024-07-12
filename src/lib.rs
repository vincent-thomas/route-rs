use std::{
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex},
};
pub mod address;

use address::Address;
use endpoint::EndpointRouter;
use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::Service};
use matchit::Router;

use hyper_util::rt::{TokioIo, TokioTimer};
mod service;
pub use route_core::*;
pub use route_derive::*;
use tokio::net::TcpListener;

pub use route_http as http;

pub struct App<S = ()> {
  inner: Arc<InnerApp<S>>,
}

impl<S> Clone for App<S> {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone() }
  }
}

struct InnerApp<T> {
  routes: Router<EndpointRouter<T>>,
  bound_address: Option<Address>,
}

impl<T> App<T>
where
  T: Clone,
{
  pub fn new() -> Self {
    Self { inner: Arc::new(InnerApp { routes: Router::default(), bound_address: None }) }
  }
  fn tap_inner_mut<F>(self, f: F) -> Self
  where
    F: FnOnce(&mut InnerApp<T>),
  {
    let mut inner = self.into_inner();
    f(&mut inner);
    Self { inner: Arc::new(inner) }
  }

  fn into_inner(self) -> InnerApp<T> {
    match Arc::try_unwrap(self.inner) {
      Ok(inner) => inner,
      Err(arc) => InnerApp { routes: arc.routes.clone(), bound_address: arc.bound_address.clone() },
    }
  }
}

impl<S> App<S>
where
  S: Clone,
{
  pub fn service(self, path: &str, route_service: EndpointRouter<S>) -> Self {
    self.tap_inner_mut(|inner| {
      inner.routes.insert(path, route_service);
    })
  }
}

impl<T> App<T>
where
  T: Send + 'static + Sync + Send,
{
  // pub fn bind(self, address: Address) -> App<T> {
  //   // App { innerroutes: self.routes, bound_address: Some(address) }
  // }
  pub async fn listen(self, port: u16) {
    let address: String =
      self.inner.bound_address.clone().expect("address is required for listening").into();
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(host).await.unwrap();

    let mutex = Mutex::new(self);

    let app = Arc::new(mutex);

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

struct Nice<T> {
  app: Arc<Mutex<App<T>>>,
}

impl<T> Service<hyper::Request<hyper::body::Incoming>> for Nice<T> {
  type Response = hyper::Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
    let app_mutex = Arc::clone(&self.app);
    let uri = req.uri().clone();

    let app = app_mutex.lock().unwrap();
    let route = match app.inner.routes.at(uri.path()) {
      Ok(route) => route,
      Err(_) => return Box::pin(async { Ok(hyper::Response::new(Full::new(Bytes::from("404")))) }),
    };

    let thing = route.value;

    let test = uri.path().to_string();
    return todo!();

    // let request = HttpRequest::from(req);
    // headers: req.headers().clone(),
    // body: "".to_string(),
    // method: HttpMethod::Get,
    // path: test,
    // variables: HashMap::new(),
    //};

    // Box::pin(async move {
    //   let response = thing.call(request).await;
    //   let res = hyper::Response::new(Full::new(Bytes::from(response.body().clone())));
    //   Ok(res)
    // })
  }
}
