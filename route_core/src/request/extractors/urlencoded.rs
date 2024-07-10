use route_http::request::HttpRequest;
use serde::de::DeserializeOwned;

use crate::request::RequestExtractor;

use super::BodyParseError;

struct UrlEncoded<T>(T);

impl<T> RequestExtractor<UrlEncoded<T>, BodyParseError> for HttpRequest
where
  T: DeserializeOwned,
{
  fn extract(&self) -> Result<UrlEncoded<T>, BodyParseError> {
    let content_type = self.headers.get("Content-Type");
    if content_type.is_none()
      || content_type.is_some_and(|v| v != "application/x-www-form-urlencoded")
    {
      return Err(BodyParseError::ContentTypeInvalid);
    }
    let body = &self.body;
    if body.is_empty() {
      return Err(BodyParseError::NoBody);
    }
    let form = serde_urlencoded::from_str(body).unwrap();
    Ok(UrlEncoded(form))
  }
}
