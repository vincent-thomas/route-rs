use std::collections::HashMap;

use crate::http::{
  header, mime, Body, FromRequest, Request, Respondable, Response,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use super::BodyParsingError;

/// A wrapper around deserialized JSON data.
///
/// The `Json` extractor is used to parse and deserialize JSON data from the request body. It is
/// typically used when you expect the body of an HTTP request to contain JSON and want to convert
/// it into a Rust type. The type `T` represents the data structure you're deserializing into. By default,
/// `T` is `HashMap<String, Value>`, but you can specify any type that implements `DeserializeOwned`.
///
/// # Example
///
/// ```
/// use titan::{web, http::Respondable};
/// use serde::{Deserialize, Serialize};
/// use serde_json::Value;
///
/// #[derive(Deserialize, Serialize, Clone)]
/// struct MyData {
///     name: String,
///     age: u32,
/// }
///
/// // Extracting JSON from the request and deserializing it into a custom type.
/// async fn handle_request(web::Json(data): web::Json<MyData>) -> impl Respondable {
///   data.name
/// }
/// ```
#[derive(Clone)]
pub struct Json<T = HashMap<String, Value>>(pub T);

impl<T> FromRequest for Json<T>
where
  T: DeserializeOwned,
{
  type Error = BodyParsingError;
  fn from_request(req: Request) -> Result<Self, Self::Error> {
    let content_type = req.headers().get("content-type");
    let body = req.body();
    if content_type.is_none()
      || content_type.is_some_and(|v| v != "application/json")
    {
      Err(BodyParsingError::ContentTypeInvalid)
    } else if body.is_empty() {
      Err(BodyParsingError::NoBody)
    } else {
      let json = serde_json::from_slice(body)
        .map_err(|x| BodyParsingError::ParsingError(x.to_string()))?;
      Ok(Json(json))
    }
  }
}

impl<W: Serialize> Serialize for Json<W> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<S: Serialize> Respondable for Json<S> {
  fn respond(self) -> Response {
    let body = serde_json::to_string(&self.0).unwrap();
    let body_len = body.len();
    let mut res = Response::new(Body::from(body));
    let headers = res.headers_mut();

    headers.insert(
      header::CONTENT_TYPE,
      mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );

    headers.insert(header::CONTENT_LENGTH, body_len.into());
    res
  }
}
