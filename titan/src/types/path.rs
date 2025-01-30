use std::convert::Infallible;

use crate::http::{request::Parts, FromRequestParts};

/// A struct representing the path component of a request's URI.
///
/// The `Path` struct is used to extract and store the path part of the URI from an incoming request.
/// This can be useful for web applications to capture the path of the URL when handling routes.
///
/// # Example
/// ```
/// use titan::web::Path;
///
/// // Extract the URI path from the request
/// async fn handle_request(Path(path_str): Path) -> String {
///   format!("Path: {path_str}")
/// }
/// ```
pub struct Path(pub String);

impl FromRequestParts for Path {
  type Error = Infallible;
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    Ok(Path(parts.uri.path().to_string()))
  }
}
