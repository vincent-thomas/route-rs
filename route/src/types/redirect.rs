use async_trait::async_trait;
use route_core::{service::HttpService, Respondable};
use route_http::{
  header, request::HttpRequest, response::HttpResponse, StatusCode,
};

pub struct Redirect {
  to: &'static str,
  status_code: StatusCode,
}

impl Redirect {
  pub fn new(to: &'static str) -> Redirect {
    Redirect { to, status_code: StatusCode::PERMANENT_REDIRECT }
  }
  fn gen_response(&self) -> HttpResponse {
    let mut res = HttpResponse::new(Box::new([]));
    *res.status_mut() = self.status_code;

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}
#[async_trait]
impl HttpService for Redirect {
  async fn call_service(&self, _req: HttpRequest) -> HttpResponse {
    let test = self.gen_response();
    dbg!(&test);
    test
    //let res = Response::builder().header("Location", );
  }
}

// impl HttpService for Redirect {
//   fn call_service(
//     &self,
//     _req: route_http::request::HttpRequest,
//   ) -> std::pin::Pin<Box<dyn std::future::Future<Output = HttpResponse> + Send + 'static>> {
//     Box::pin(async move { self.gen_response() })
//   }
// }

impl Respondable for Redirect {
  fn respond(self) -> route_http::response::HttpResponse {
    self.gen_response()
  }
}
