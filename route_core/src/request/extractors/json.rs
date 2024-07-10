use route_http::request::HttpRequest;
use serde::de::DeserializeOwned;

use crate::request::RequestExtractor;

use super::BodyParseError;

struct Json<T>(T);

impl<T> RequestExtractor<Json<T>, BodyParseError> for HttpRequest
where
  T: DeserializeOwned,
{
  fn extract(&self) -> Result<Json<T>, BodyParseError> {
    let content_type = self.headers.get("Content-Type");
    if content_type.is_none() || content_type.is_some_and(|v| v != "application/json") {
      return Err(BodyParseError::ContentTypeInvalid);
    }
    let body = &self.body;
    if body.is_empty() {
      return Err(BodyParseError::NoBody);
    }
    let json = serde_json::from_str(body).unwrap();
    Ok(Json(json))
  }
}
