use route_http::{body::Body, response::Response, StatusCode};

pub trait Respondable {
  fn respond(self) -> Response<Body>;
}

impl<T, E> Respondable for Result<T, E>
where
  T: Respondable,
  E: Respondable,
{
  fn respond(self) -> Response<Body> {
    match self {
      Ok(t) => t.respond(),
      Err(e) => e.respond(),
    }
  }
}

impl Respondable for Response<Body> {
  fn respond(self) -> Response<Body> {
    self
  }
}

impl Respondable for () {
  fn respond(self) -> Response<Body> {
    Response::new(Body)
  }
}

impl<T> Respondable for (StatusCode, T)
where
  T: Respondable,
{
  fn respond(self) -> Response<Body> {
    let (status, body) = self;
    let mut res = body.respond();

    *res.status_mut() = status;
    res
  }
}

macro_rules! impl_respondable_for_int {
    ($($t:ty)*) => {
        $(
          impl Respondable for $t {
            fn respond(self) -> Response<Body> {
              let mut res = Response::new(Body);
              let headers = res.headers_mut();
              headers.insert("content-type", "text/plain".parse().unwrap());
              res
            }
          }
        )*
    };
}

impl_respondable_for_int!(String &str i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize);
