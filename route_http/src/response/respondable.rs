use std::fmt::Debug;

use crate::HttpRequest;

use super::HttpResponse;

pub trait Respondable
where
  Self: Send + Sync,
{
  fn respond(self, req: &HttpRequest) -> HttpResponse;
}

impl<T, E> Respondable for Result<T, E>
where
  T: Respondable,
  E: Debug + Sync + Send + Clone,
{
  fn respond(self, req: &HttpRequest) -> HttpResponse {
    match self {
      Ok(t) => t.respond(req),
      Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
  }
}
impl<T> Respondable for Option<T>
where
  T: Respondable,
{
  fn respond(self, req: &HttpRequest) -> HttpResponse {
    match self {
      Some(t) => t.respond(req),
      None => HttpResponse::NoContent(),
    }
  }
}

impl Respondable for HttpResponse {
  fn respond(self, _req: &HttpRequest) -> HttpResponse {
    self
  }
}

impl Respondable for String {
  fn respond(self, _req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body(self)
  }
}

macro_rules! impl_respondable {
  ($($t:ty),*) => {
    $(
      impl Respondable for $t {
        fn respond(self, _req: &HttpRequest) -> HttpResponse {
          HttpResponse::Ok().body(self.to_string())
        }
      }
    )*
  };
}

impl_respondable!(&str, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool);
