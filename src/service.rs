use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use http_body_util::Full;
use hyper::{
  body::{Bytes, Incoming as IncomingBody},
  Request, Response,
};
use route_http::{method::HttpMethod, request::HttpRequestBuilder};

use crate::handler::Handler;

#[derive(Clone)]
pub struct MainService<T>
where
  T: Send,
{
  app: Arc<T>,
}

impl<T> MainService<T>
where
  T: Send,
{
  pub fn new(app: Arc<T>) -> Self {
    MainService { app }
  }
}

// impl<T> MainService<T>
// where
//   T: Handler,
// {
//   fn format_method(req: &Request<IncomingBody>) -> HttpMethod {
//     //   let path = req.uri().path();
//     match req.method().as_str() {
//       "GET" => HttpMethod::Get,
//       "POST" => HttpMethod::Post,
//       "DELETE" => HttpMethod::Delete,
//       _ => unimplemented!(),
//     }
//   }
// }

impl<T> hyper::service::Service<Request<IncomingBody>> for MainService<T>
where
  T: Handler + Send + Sync,
{
  type Response = Response<Full<Bytes>>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn call(&self, req: Request<IncomingBody>) -> Self::Future {
    // let body = req.body();
    //let path = req.uri().path();

    let _req = HttpRequestBuilder {
      headers: req.headers().clone(),
      body: "",
      path: "/",
      variables: HashMap::new(),
      method: HttpMethod::Get,
    }
    .build();

    let app = Arc::clone(&self.app);

    Box::pin(async move {
      Ok(Response::new(Full::new(Bytes::from("Hello World"))))
      // let result = <App<T> as Clone>::clone(&app).call(req).await;
      //
      // let response = Response::new(Full::new(result.body));
      //
      // Ok(response)
    })

    //
    // let route = self.app.routes.(method.clone(), url);
    //
    // let output = match route {
    //   Ok(route) => {
    //     let test2 = HttpRequest { variables: route.variables };
    //
    //     Box::pin(async {
    //       let test = route.route.handler.call(test2).await;
    //       let test = test.body();
    //       Ok(Response::new(Full::new(Bytes::from(test))))
    //     }) as Self::Future
    //   }
    //   Err(_) => Box::pin(async {
    //     let response_builder = Response::builder().status(404).body("404 Not found");
    //     return response_builder.unwrap();
    //   }) as Self::Future,
    // };
    // output
  }
}
