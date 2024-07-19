use route_http::request::HttpRequest;

use crate::error::Error;

pub trait FromRequest: Sized {
  type Error: Into<Error>;
  fn from_request(req: HttpRequest) -> Result<Self, Self::Error>;
  fn extract(req: HttpRequest) -> Result<Self, Self::Error> {
    Self::from_request(req)
  }
}

// impl<T, B> FromRequest for (T, B)
// where
//   T: FromRequest,
//   B: FromRequest,
//   T::Error: Into<Error>,
// {
//   type Error = Error;
//   type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
//   fn from_request(req: HttpRequest) -> Self::Future {
//     Box::pin(async move {
//       let fut = async move {
//         let arg = (
//           match T::from_request(req.clone()).await {
//             Ok(v) => v,
//             Err(_) => unimplemented!(),
//           },
//           match B::from_request(req.clone()).await {
//             Ok(v) => v,
//             Err(_) => unimplemented!(),
//           },
//         );
//         Ok(arg)
//       };
//       fut.await
//     })
//   }
// }

macro_rules! impl_fromrequest_tuple {
  ($($name:ident),*) => {
    impl<$($name: FromRequest),*> FromRequest for ($($name,)*) {
      type Error = $crate::error::Error;
      #[allow(unused_variables)]
      fn from_request(req: HttpRequest) -> Result<Self, Self::Error> {
        // Box::pin(async move {
          // let fut = async move {
            let args = ($(match $name::from_request(req.clone()) {Ok(v) => v, Err(_) => unimplemented!(),},)*);
            Ok(args)
          // };
          // fut.await
        // })
      }
    }
  };
}

impl_fromrequest_tuple!();
impl_fromrequest_tuple!(A);
impl_fromrequest_tuple!(A, B);
impl_fromrequest_tuple!(A, B, C);
impl_fromrequest_tuple!(A, B, C, D);
impl_fromrequest_tuple!(A, B, C, D, E);
impl_fromrequest_tuple!(A, B, C, D, E, F);
impl_fromrequest_tuple!(A, B, C, D, E, F, G);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

// macro_rules! impl_fromrequest_tuple {
//   ($($name:ident),*) => {
//     impl<$($name: FromRequest),*> FromRequest for ($($name,)*) {
//       type Error = $crate::error::Error;
//       type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
//       fn from_request(req: HttpRequest) -> Self::Future {
//         Box::pin(async move {
//           let fut = async move {
//             let args = ($($name::from_request(req).await?,)*);
//             Ok(args)
//           };
//           fut.await
//         })
//       }
//     }
//   };
// }

// impl_fromrequest_tuple!();
// impl_fromrequest_tuple!(A);
// //impl_fromrequest_tuple!(A, B);
// impl_fromrequest_tuple!(A, B, C);
// impl_fromrequest_tuple!(A, B, C, D);
// impl_fromrequest_tuple!(A, B, C, D, E);
// impl_fromrequest_tuple!(A, B, C, D, E, F);
// impl_fromrequest_tuple!(A, B, C, D, E, F, G);
// impl_fromrequest_tuple!(A, B, C, D, E, F, G, H);

// pub trait FromRequestParts: Sized {
//   type Error;
//   type Future: Future<Output = Result<Self, Self::Error>>;
//   fn from_request_parts(req: &Head) -> Self::Future;
//   fn extract_parts(req: &Head) -> Self::Future {
//     Self::from_request_parts(req)
//   }
// }
