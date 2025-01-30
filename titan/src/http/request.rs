use crate::all_the_tuples;
use crate::http::{Respondable, Response};
use std::convert::Infallible;

pub use http::request::Builder;
pub use http::request::Parts;

pub type Request = http::Request<Box<[u8]>>;

/// Types that can be extracted from a request.
///
/// The `FromRequest` trait is used for types that can be constructed from an HTTP request. Extractors
/// implementing this trait are responsible for parsing the incoming request body, headers, or other
/// parts of the request, and converting them into a structured type that can be used by handlers.
///
/// Since `FromRequest` extractors consume the request body, they can only be executed once for each
/// handler. If your extractor doesn't need to consume the request body (for example, when you only
/// need access to the request headers or parts of the URL), you should implement [`FromRequestParts`]
/// instead of [`FromRequest`].
///
/// # Associated Types
/// - `Error`: The type of error that can occur while extracting the request data. It must implement
///   the [`Respondable`] trait to allow the error to be returned as a valid HTTP response.
///
/// This method extracts the data from the given HTTP request and returns a result. If the extraction
/// is successful, it returns `Ok(Self)`, where `Self` is the type implementing `FromRequest`. If
/// an error occurs during extraction (e.g., due to invalid data or missing fields), it returns an
/// error of type `Self::Error`, which will be transformed into an HTTP response via the `Respondable`
/// trait.
///
/// # Example
///
/// ```
/// use titan_core::{FromRequest, Respondable};
/// use titan_http::{Request, body::Body};
///
/// // A custom extractor type that implements `FromRequest`.
/// struct MyExtractor {
///     pub field: String,
/// }
///
/// // Implement `FromRequest` for `MyExtractor`.
/// impl FromRequest for MyExtractor {
///     type Error = String; // The error type we will return if extraction fails.
///
///     fn from_request(req: Request) -> Result<Self, Self::Error> {
///         // Attempt to extract the data from the request (e.g., reading the body).
///         let field_value = req.uri().path().to_string(); // Example of extracting from the URL path.
///         Ok(MyExtractor { field: field_value })
///     }
/// }
///
/// async fn handler(data: MyExtractor) -> impl titan_core::Respondable { /* ... */}
///
/// // Now, `MyExtractor` can be used in a handler to extract data from the request.
/// ```
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
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let _ = req;
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

#[cfg(feature = "lambda")]
impl FromRequest for lambda_http::http::Request<lambda_http::Body> {
  type Error = ();
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let (parts, body) = req.into_parts();

    Ok(lambda_http::http::Request::from_parts(
      parts,
      lambda_http::Body::Binary(body.to_vec()),
    ))
  }
}
