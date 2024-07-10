use std::{future::Future, pin::Pin};

use route_http::{method::HttpMethod, response::HttpResponse};

use crate::handler::Handler;

pub struct EndpointRouter<R = ()> {
  get: Option<R>,
  post: Option<R>,
  put: Option<R>,
  patch: Option<R>,
  delete: Option<R>,
}

impl<R> Default for EndpointRouter<R> {
  fn default() -> Self {
    EndpointRouter { get: None, post: None, put: None, patch: None, delete: None }
  }
}

macro_rules! add_method {
  ($name:ident) => {
    pub fn $name(mut self, handler: R) -> Self {
      self.$name = Some(handler);
      self
    }
  };
}

impl<R> EndpointRouter<R>
where
  R: Handler,
{
  add_method!(get);
  add_method!(post);
  add_method!(put);
  add_method!(patch);
  add_method!(delete);
}

impl<R> Handler for EndpointRouter<R>
where
  R: Handler,
{
  type Future = Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
  fn call(self, req: route_http::request::HttpRequest) -> Self::Future {
    let output = async {
      match req.method {
        HttpMethod::Get => EndpointRouter::check_and_if_there_run(self.get, req).await,
        HttpMethod::Post => EndpointRouter::check_and_if_there_run(self.post, req).await,
        HttpMethod::Put => EndpointRouter::check_and_if_there_run(self.put, req).await,
        HttpMethod::Patch => EndpointRouter::check_and_if_there_run(self.patch, req).await,
        HttpMethod::Delete => EndpointRouter::check_and_if_there_run(self.delete, req).await,
        HttpMethod::Head => EndpointRouter::fix_head(self, req).await,
      }
    };
    Box::pin(output)
  }
}

impl<R> EndpointRouter<R>
where
  R: Handler,
{
  async fn check_and_if_there_run(
    real: Option<R>,
    req: route_http::request::HttpRequest,
  ) -> HttpResponse {
    match real {
      Some(nice) => nice.call(req).await,
      None => {
        HttpResponse::MethodNotAllowed().content_type("text/plain").body("Method not allowed")
      }
    }
  }

  async fn fix_head(self, req: route_http::request::HttpRequest) -> HttpResponse {
    let exists = self.get;

    match exists {
      Some(nice) => {
        let mut output = nice.call(req).await;
        output.body = "".to_string().into();
        output
      }
      None => HttpResponse::NotFound(),
    }
  }
}
