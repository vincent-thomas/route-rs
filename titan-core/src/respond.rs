use std::convert::Infallible;

use titan_http::{body::Body, header, Response, ResponseBuilder, StatusCode};

/// A trait for types that can be converted into an HTTP response.
///
/// The `Respondable` trait is intended for types that can be transformed into a valid HTTP response.
/// Any type returned by a handler (or any function) that implements this trait can be converted into
/// a `Response<Body>`. This is typically used to ensure that different types can be unified into
/// a standard response format for HTTP APIs or services.
///
/// This method converts the implementing type into an HTTP response represented by a `Response<Body>`.
/// It allows various types (e.g., structs, enums, or other custom types) to be returned from a handler
/// and automatically converted into HTTP responses.
///
/// # Implementing `Respondable`
///
/// To implement the `Respondable` trait, you need to define how your custom type can be turned into
/// an HTTP response. This is commonly done by converting your type into a response body (e.g., a string,
/// JSON, or some binary data) and returning it wrapped in a `Response<Body>`.
///
/// # Example
///
/// ```
/// use titan_http::{body::Body, Response};
/// use titan_core::Respondable;
///
/// // Define a type that represents a response body.
/// pub struct MyResponse {
///     message: String,
/// }
///
/// // Implement `Respondable` for `MyResponse`.
/// impl Respondable for MyResponse {
///     fn respond(self) -> Response<Body> {
///         // Convert the struct into an HTTP response with the message in the body.
///         Response::new(Body::from(self.message))
///     }
/// }
///
/// // Now you can return `MyResponse` from a handler, and it will be automatically
/// // converted into an HTTP response.
/// ```
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

impl Respondable for Infallible {
  fn respond(self) -> Response<Body> {
    panic!("Not fallible :(")
  }
}

impl Respondable for () {
  fn respond(self) -> Response<Body> {
    ResponseBuilder::new().status(204).body(Body::from(())).unwrap()
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

impl Respondable for titan_html::tags::html::Html {
  fn respond(self) -> Response<Body> {
    let response = ResponseBuilder::new().status(200);

    let mut head_response = response.header(header::CONTENT_TYPE, "text/html");

    if let Some(nonce) = self.with_csp_nonce.clone() {
      head_response
        .headers_mut()
        .unwrap()
        .insert(header::CONTENT_SECURITY_POLICY, format!("script-src 'self' 'nonce-{nonce}'; style-src 'self' 'nonce-{nonce}';").parse().unwrap());
    }

    let str = titan_html::render(self);
    head_response.body(Body::from(str)).unwrap()
  }
}

macro_rules! impl_respondable_for_int {
    ($($t:ty)*) => {
        $(
          impl Respondable for $t {
            fn respond(self) -> Response<Body> {
              let body = Body::from(self);

              let mut res = Response::new(body);
              let headers = res.headers_mut();

              headers.insert("content-type", "text/plain".parse().unwrap());

              res
            }
          }
        )*
    };
}

impl_respondable_for_int!(String &str i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize);
