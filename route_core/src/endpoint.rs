use async_trait::async_trait;
use route_http::{request::HttpRequest, response::HttpResponse};
use std::future::Future;

use crate::Respondable;

#[async_trait]
pub trait Endpoint: Send + Sync + 'static {
  async fn call(&self, req: HttpRequest) -> HttpResponse;
}

#[async_trait]
impl<F, Fut, O> Endpoint for F
where
  F: Send + Sync + 'static + Fn(HttpRequest) -> Fut,
  Fut: Future<Output = O> + Send + 'static,
  O: Respondable + 'static,
{
  async fn call(&self, req: HttpRequest) -> HttpResponse {
    let func = (self)(req).await;
    func.respond()
  }
}

pub type BoxedEndpoint = Box<dyn Endpoint>;
