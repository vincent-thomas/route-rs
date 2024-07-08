use std::{convert::Infallible, fmt::Debug, future::Future, pin::Pin, sync::Arc};

use route_contract::{method::HttpMethod, respondable::Respondable, HttpRequest};
use route_router::{FromRequest, Route, Router};

use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
pub use route_derive::*;
use tokio::net::TcpListener;

pub struct App<Address, Res>
where
  Address: Clone,
  Res: Respondable + Debug,
{
  routes: Router<HttpRequest, Res>,
  bound_address: Address,
}

#[derive(Clone)]
pub struct NoBoundAddress;
#[derive(Clone)]
pub struct Address(pub u8, pub u8, pub u8, pub u8);

impl Into<String> for Address {
  fn into(self) -> String {
    format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3)
  }
}

pub trait Service {
  fn method(&self) -> HttpMethod;
  fn path(&self) -> String;
  // fn handler(&self) -> impl Respondable;
}

impl<R> Default for App<NoBoundAddress, R>
where
  R: Respondable + Debug,
{
  fn default() -> Self {
    App { routes: Router::mount_root(), bound_address: NoBoundAddress }
  }
}

impl<R> App<NoBoundAddress, R>
where
  R: Respondable + Debug,
{
  pub fn mount_at(path: String) -> Self {
    App { routes: Router::mount_at(path), bound_address: NoBoundAddress }
  }

  pub fn service(&mut self, service: impl Service, handler: fn(req: HttpRequest) -> R) {
    let method = service.method();
    let path = service.path();
    self.routes.route(method, path, Route::new(handler));
  }
  pub fn bind(self, address: Address) -> App<Address, R> {
    App { routes: self.routes, bound_address: address }
  }
}

impl<R> App<Address, R>
where
  R: Debug + Respondable,
{
  pub async fn listen(self, port: u16) {
    let address: String = self.bound_address.clone().into();
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(host).await.unwrap();

    let app = Arc::new(self);

    loop {
      let (tcp, _) = listener.accept().await.unwrap();
      let io = TokioIo::new(tcp);

      // tokio::task::spawn(async move {
      let result =
        http1::Builder::new().serve_connection(io, MainService { app: Arc::clone(&app) }).await;

      println!("{:?}", result);
      // service_fn(move |req: Request<hyper::body::Incoming>| -> Result<Response<Full<Bytes>>, Infallible> {
      //   let app = Arc::clone(&app);

      //   Ok(Response::new(Full::new(Bytes::from("404 Not found"))))
      //   // let route = app.routes.match_route(method, path);
      //   // return async {
      //   //   match route {
      //   //     Ok(route) => {
      //   //       let test2 = HttpRequest {};
      //   //       let test = &(route.route.handler)(test2);

      //   //       let test = test.clone().body();

      //   //       Ok(Response::new(Full::new(Bytes::from(test))))
      //   //     }
      //   //     Err(_) => Ok(Response::new(Full::new(Bytes::from("404 Not found")))),
      //   //   }
      //   // };
      //   //hello(app)(req)
      // })

      // let app =
      // Finally, we bind the incoming connection to our `hello` service
      // if let Err(err) = http1::Builder::new()
      //   // `service_fn` converts our function in a `Service`
      //   .serve_connection(io, service_fn(hello(app_to_send)))
      //   .await
      // {
      //   eprintln!("Error serving connection: {:?}", err);
      // }
      // });
    }
  }
}

#[derive(Clone)]
struct MainService<R>
where
  R: Respondable + Debug,
{
  app: Arc<App<Address, R>>,
}

impl<R> MainService<R>
where
  R: Debug + Respondable,
{
  fn format_method(req: &Request<IncomingBody>) -> HttpMethod {
    //   let path = req.uri().path();
    match req.method().as_str() {
      "GET" => HttpMethod::Get,
      "POST" => HttpMethod::Post,
      "DELETE" => HttpMethod::Delete,
      _ => unimplemented!(),
    }
  }
}

use hyper::body::Incoming as IncomingBody;

impl<R> hyper::service::Service<Request<IncomingBody>> for MainService<R>
where
  R: Debug + Respondable,
{
  type Response = Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, req: Request<IncomingBody>) -> Self::Future {
    let url = req.uri().path();

    let method = MainService::<R>::format_method(&req);

    let route = self.app.routes.match_route(method.clone(), url);

    let output = match route {
      Ok(route) => {
        let test2 = HttpRequest { variables: route.variables };
        let test = &(route.route.handler)(test2);

        let test = test.clone().body();

        Ok(Response::new(Full::new(Bytes::from(test))))
      }
      Err(_) => Ok(Response::new(Full::new(Bytes::from("404 Not found")))),
    };

    Box::pin(async { output })
  }
}

// #[derive(Clone, Debug)]
// pub struct HttpRequest;

// impl FromRequest for HttpRequest {}
// fn hello<R: Respondable>(
//   app: Arc<App<Address, R>>,
// ) -> impl Fn(
//   Request<hyper::body::Incoming>,
// ) -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>, Infallible>>>>
//      + Send
//      + Sync {
//   move |test| {
//     let app = Arc::clone(&app);
//     let result = async move {
//       let path = test.uri().path();
//       let method = match test.method().as_str() {
//         "GET" => HttpMethod::Get,
//         "POST" => HttpMethod::Post,
//         "DELETE" => HttpMethod::Delete,
//         _ => unimplemented!(),
//       };

//       let route = app.routes.match_route(method, path);

//       match route {
//         Ok(route) => {
//           let test2 = HttpRequest {};
//           let test = &(route.route.handler)(test2);

//           let test = test.body();

//           Ok(Response::new(Full::new(Bytes::from(test))))
//         }
//         Err(_) => Ok(Response::new(Full::new(Bytes::from("404 Not found")))),
//       }
//     };
//     Box::pin(result)
//   }
//   // async move |test| {
//   //   let path = test.uri().path();

//   //   let route = app.routes.match_route(method, path);
//   //   dbg!(route);
//   // }
// }
