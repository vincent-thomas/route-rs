use std::{
  error::Error,
  io::{Read, Write},
  pin::Pin,
};

use route_http::{
  header::{HeaderMap, HeaderValue},
  response::{Head, HttpResponse, HttpResponseExt},
  StatusCode,
};

pub trait Respondable {
  fn respond(self) -> HttpResponse;
}

// #[async_trait::async_trait]
// pub trait RespondableV2 {
//   fn head(&self) -> route_http::response::Head;
//   async fn body<T>(self, stream: &mut T) -> Result<(), Box<dyn Error>>
//   where
//     T: Write + Send;
// }
//
// #[async_trait::async_trait]
// impl<T: Respondable + Send> RespondableV2 for T {
//   fn head(&self) -> route_http::response::Head {
//     let headers: HeaderMap<HeaderValue> = HeaderMap::new();
//     Head { status: StatusCode::OK, headers }
//   }
//
//   async fn body<W>(self, stream: &mut W) -> Result<(), Box<dyn Error>>
//   where
//     W: Write + Send,
//   {
//     let res = self.respond();
//
//     let raw_body: String = HttpResponseExt(res).into();
//     stream.write_all(raw_body.as_bytes())?;
//     stream.flush()?;
//     Ok(())
//   }
// }

impl<T, E> Respondable for Result<T, E>
where
  T: Respondable,
  E: Respondable,
{
  fn respond(self) -> HttpResponse {
    match self {
      Ok(t) => t.respond(),
      Err(e) => e.respond(),
    }
  }
}

impl Respondable for HttpResponse {
  fn respond(self) -> HttpResponse {
    self
  }
}

impl Respondable for () {
  fn respond(self) -> HttpResponse {
    HttpResponse::new(vec![].into())
  }
}

impl<T> Respondable for (StatusCode, T)
where
  T: Respondable,
{
  fn respond(self) -> HttpResponse {
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
            fn respond(self) -> HttpResponse {
              let mut res = HttpResponse::new(self.to_string().as_bytes().into());
              let headers = res.headers_mut();
              headers.insert("content-type", "text/plain".parse().unwrap());
              res
            }
          }
        )*
    };
}

impl_respondable_for_int!(String &str i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize);
