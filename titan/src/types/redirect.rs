use std::task::Poll;

use crate::{
  http::{header, Body, Request, Respondable, Response, StatusCode},
  BoxedSendFuture,
};
use tower::Service;

/// A struct representing an HTTP redirect.
///
/// The `Redirect` struct is used to generate HTTP responses for redirects. It supports both
/// permanent (`301 Moved Permanently`) and temporary (`302 Found`) redirects. The redirect
/// target is specified by a URL, and the response will include the `Location` header pointing to
/// that URL.
///
/// # Example
///
/// ```no_run
/// use titan::{web, App, web::Redirect};
///
/// // Create a permanent redirect
/// let redirect = Redirect::permanent("https://example.com");
///
/// // Create a temporary redirect
/// let _ = Redirect::temporary("https://example.com");
///
/// let app = App::default().at("to-google", redirect);
/// // ...
/// ```
#[derive(Clone)]
pub struct Redirect {
  to: &'static str,
  permanent: bool,
}

impl Redirect {
  /// Creates a permanent (`301 Moved Permanently`) redirect to the specified URL.
  ///
  /// # Arguments
  /// - `to`: The target URL for the redirect, which must be a valid URL string.
  ///
  /// # Returns
  /// [`Redirect`] with permanent redirection
  pub fn permanent(to: &'static str) -> Redirect {
    Redirect { permanent: true, to }
  }
  /// Creates a temporary (`302 Moved Temporarily`) redirect to the specified URL.
  ///
  /// # Arguments
  /// - `to`: The target URL for the redirect, which must be a valid URL string.
  ///
  /// # Returns
  /// [`Redirect`] with temporary redirection
  pub fn temporary(to: &'static str) -> Redirect {
    Redirect { permanent: false, to }
  }

  fn gen_response(&self) -> Response {
    let mut res = Response::new(Body::from(()));
    *res.status_mut() = if self.permanent {
      StatusCode::PERMANENT_REDIRECT
    } else {
      StatusCode::TEMPORARY_REDIRECT
    };

    let headers = res.headers_mut();

    if let Ok(location) = self.to.parse() {
      headers.insert(header::LOCATION, location);
    }

    res
  }
}

impl Service<Request> for Redirect {
  type Response = Response;
  type Error = Response;
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, _req: Request) -> Self::Future {
    let response = self.gen_response();
    Box::pin(async move { Ok(response) })
  }
}

impl Respondable for Redirect {
  fn respond(self) -> Response {
    self.gen_response()
  }
}
