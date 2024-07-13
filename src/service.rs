use std::{pin::Pin, sync::Arc};

use http_body_util::Full;
use hyper::{body::Bytes, service::Service};
use std::future::Future;

use crate::App;
struct AppService {
  app: Arc<App>,
}

impl Service<hyper::Request<hyper::body::Incoming>> for AppService {
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
