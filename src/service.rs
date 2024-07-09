use std::{future::Future, pin::Pin, sync::Arc};

use http_body_util::Full;
use hyper::{
  body::{Bytes, Incoming as IncomingBody},
  Request, Response,
};
use route_http::{method::HttpMethod, HttpRequest};

use crate::{Address, App};

#[derive(Clone)]
pub struct MainService {
  app: Arc<App<Address>>,
}

impl MainService {
  pub fn new(app: Arc<App<Address>>) -> Self {
    MainService { app }
  }
}

impl MainService {
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

impl hyper::service::Service<Request<IncomingBody>> for MainService {
  type Response = Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, req: Request<IncomingBody>) -> Self::Future {
    let url = req.uri().path();

    let method = MainService::format_method(&req);

    let route = self.app.routes.match_route(method.clone(), url);

    let output = match route {
      Ok(route) => {
        let test2 = HttpRequest { variables: route.variables };

        Box::pin(async {
          let test = route.route.handler.call(test2).await;
          let test = test.body();
          Ok(Response::new(Full::new(Bytes::from(test))))
        }) as Self::Future
      }
      Err(_) => Box::pin(async {
        let response_builder = Response::builder().status(404).body("404 Not found");
        return response_builder.unwrap();
      }) as Self::Future,
    };
    output
  }
}
