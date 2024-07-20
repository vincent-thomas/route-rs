use route_http::{request::HttpRequest, response::HttpResponse};

#[async_trait::async_trait]
pub trait HttpService: Sync + Send {
  async fn call_service(&self, req: HttpRequest) -> HttpResponse;
}
