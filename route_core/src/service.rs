use route_http::{request::HttpRequest, response::HttpResponse};
use std::{future::Future, pin::Pin};

pub trait HttpService {
  fn call_service(
    &'static self,
    req: HttpRequest,
  ) -> Pin<Box<dyn Future<Output = HttpResponse> + 'static>>;
}
