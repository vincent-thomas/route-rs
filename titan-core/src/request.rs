use crate::Respondable;
use std::convert::Infallible;
use titan_http::{
  request::{Parts, Request},
  response::Response,
};

/// Types that can be created from requests.
///
/// Extractors that implement `FromRequest` can consume the request body and can thus only be run
/// once for handlers.
///
/// If your extractor doesn't need to consume the request body then you should implement
/// [`FromRequestParts`] and not [`FromRequest`].
///
///
pub trait FromRequest: Sized {
  type Error: Respondable;
  fn from_request(req: Request) -> Result<Self, Self::Error>;
}

/// Types that can be created from request parts.
///
/// Extractors that implement `FromRequestParts` cannot consume the request body and can thus be
/// run in any order for handlers.
///
/// If your extractor needs to consume the request body then you should implement [`FromRequest`]
/// and not [`FromRequestParts`].
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

impl FromRequest for Request {
  type Error = Infallible;
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    Ok(req)
  }
}

impl FromRequest for () {
  type Error = Infallible;
  fn from_request(_: Request) -> Result<Self, Self::Error> {
    Ok(())
  }
}

impl FromRequestParts for () {
  type Error = Infallible;
  fn from_request_parts(_: &mut Parts) -> Result<Self, Self::Error> {
    Ok(())
  }
}

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
