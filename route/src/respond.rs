use route_http::{response::Response, StatusCode};

pub trait Respondable {
  fn respond(self) -> Response<Box<[u8]>>;
}

//pub trait RespondableV2 {
//  type Body: MessageBody + 'static;
//
//  fn respond(self) -> Response<Self::Body>;
//}

//impl<B> RespondableV2 for Response<B> where B: MessageBody + 'static {
//    type Body = B;
//
//    #[inline]
//    fn respond(self) -> Response<Self::Body> {
//        self
//    }
//
//}

// impl RespondableV2 for BoxBody {
//   type Body = BoxBody;
//   fn respond(self) -> Response<Self::Body> {
//     Response::new(self)
//   }
// }
//
// impl RespondableV2 for Response<BoxBody> {
//   type Body = BoxBody;
//   fn respond(self) -> Response<Self::Body> {
//     self
//   }
// }

// impl<M: MessageBody + 'static> RespondableV2 for M {
//   type Body = M;
//
//   fn respond(self) -> Response<Self::Body> {
//     let body = BoxBody::Stream(Box::pin(self));
//
//     Response::new(body)
//   }
// }

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
//     let raw_body: String = HttpResponse<Box<[u8]>>Ext(res).into();
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
  fn respond(self) -> Response<Box<[u8]>> {
    match self {
      Ok(t) => t.respond(),
      Err(e) => e.respond(),
    }
  }
}

impl Respondable for Response<Box<[u8]>> {
  fn respond(self) -> Response<Box<[u8]>> {
    self
  }
}

impl Respondable for () {
  fn respond(self) -> Response<Box<[u8]>> {
    Response::new(vec![].into())
  }
}

impl<T> Respondable for (StatusCode, T)
where
  T: Respondable,
{
  fn respond(self) -> Response<Box<[u8]>> {
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
            fn respond(self) -> Response<Box<[u8]>> {
              let mut res = Response::new(self.to_string().as_bytes().into());
              let headers = res.headers_mut();
              headers.insert("content-type", "text/plain".parse().unwrap());
              res
            }
          }
        )*
    };
}

impl_respondable_for_int!(String &str i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize);
