use route_http::{mime, request::HttpRequest, response::HttpResponse, status::StatusCode};
use std::str::FromStr;

pub trait Respondable {
  fn respond(self, req: &HttpRequest) -> HttpResponse;
}

impl<T, E> Respondable for Result<T, E>
where
  T: Respondable,
  E: Respondable,
{
  fn respond(self, req: &HttpRequest) -> HttpResponse {
    match self {
      Ok(t) => t.respond(req),
      Err(e) => e.respond(req),
    }
  }
}

impl Respondable for HttpResponse {
  fn respond(self, _req: &HttpRequest) -> HttpResponse {
    self
  }
}

impl<T> Respondable for (StatusCode, T)
where
  T: Respondable,
{
  fn respond(self, req: &HttpRequest) -> HttpResponse {
    let (status, body) = self;
    let mut res = body.respond(req);
    *res.status_mut() = status;
    res
  }
}

macro_rules! impl_respondable_for_int {
    ($($t:ty)*) => {
        $(
        impl Respondable for $t {
            fn respond(self, req: &HttpRequest) -> HttpResponse {
            let mut res = HttpResponse::new(self.to_string());
            let headers = res.headers_mut();
            headers.insert("content-type", "text/plain".parse().unwrap());
            if req.headers().get("accept").is_some_and(|accepts| {
                mime::Mime::from_str(accepts.to_str().unwrap()).unwrap() != mime::TEXT_PLAIN
            }) {
                *res.body_mut() = "Unsupported Media Type".to_string();
            }
            res
            }
        }
        )*
    };
}

impl_respondable_for_int!(String &str i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize);
