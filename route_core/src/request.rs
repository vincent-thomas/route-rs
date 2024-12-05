use std::convert::Infallible;

use route_http::request::{Parts, Request};

use crate::Respondable;

use route_http::response::Response;

pub trait FromRequest: Sized {
  type Error: Respondable;
  fn from_request(req: Request) -> Result<Self, Self::Error>;
  fn extract(req: Request) -> Result<Self, Self::Error> {
    Self::from_request(req)
  }
}

pub trait FromRequestParts: Sized {
  type Error: Respondable;
  fn from_request_parts(req: &mut Parts) -> Result<Self, Self::Error>;
}

impl FromRequest for String {
  type Error = ();
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let test = req.into_body();
    Ok(String::from_utf8_lossy(&test).to_string())
  }
}

impl FromRequestParts for () {
  type Error = Infallible;
  fn from_request_parts(_: &mut Parts) -> Result<Self, Self::Error> {
    Ok(())
  }
}

//impl<T> FromRequest for T
//where
//  T: FromRequestParts,
//{
//  type Error = T::Error;
//  fn from_request(req: Request) -> Result<Self, Self::Error> {
//    let (mut parts, _) = req.into_parts();
//    Self::from_request_parts(&mut parts)
//  }
//}

macro_rules! impl_from_request {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<$($ty,)* $last> FromRequestParts for ($($ty,)* $last,)
        where
            $( $ty: FromRequestParts + Send, )*
            $last: FromRequestParts + Send,
        {
            type Error = Response;

            fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
                $(
                    let $ty = $ty::from_request_parts(parts)
                        .map_err(|err| err.respond())?;
                )*
                let $last = $last::from_request_parts(parts)
                    .map_err(|err| err.respond())?;

                Ok(($($ty,)* $last,))
            }
        }

        // This impl must not be generic over M, otherwise it would conflict with the blanket
        // implementation of `FromRequest<S, Mut>` for `T: FromRequestParts<S>`.
        #[allow(non_snake_case, unused_mut, unused_variables)]
        impl<$($ty,)* $last> FromRequest for ($($ty,)* $last,)
        where
            $( $ty: FromRequestParts + Send, )*
            $last: FromRequest + Send,
        {
            type Error = Response;

            fn from_request(req: Request) -> Result<Self, Self::Error> {
                let (mut parts, body) = req.into_parts();

                $(
                    let $ty = $ty::from_request_parts(&mut parts).map_err(|err| err.respond())?;
                )*

                let req = Request::from_parts(parts, body);

                let $last = $last::from_request(req).map_err(|err| err.respond())?;

                Ok(($($ty,)* $last,))
            }
        }
    };
}

all_the_tuples!(impl_from_request);

//macro_rules! impl_fromrequest_tuple {
//  ($($name:ident),*) => {
//    impl<$($name: FromRequest),*> FromRequest for ($($name,)*) {
//      type Error = Response;
//      #[allow(unused_variables)]
//      fn from_request(req: Request) -> Result<Self, Self::Error> {
//            let args = ($(match $name::from_request(req.clone()) {Ok(v) => v, Err(err) => return Err(err.respond())},)*);
//            Ok(args)
//      }
//    }
//  };
//}

//impl_fromrequest_tuple!();
//impl_fromrequest_tuple!(A);
//impl_fromrequest_tuple!(A, B);
//impl_fromrequest_tuple!(A, B, C);
//impl_fromrequest_tuple!(A, B, C, D);
//impl_fromrequest_tuple!(A, B, C, D, E);
//impl_fromrequest_tuple!(A, B, C, D, E, F);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G, H);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K);
//impl_fromrequest_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
