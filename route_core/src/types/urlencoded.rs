use route_http::{request::HttpRequest, response::HttpResponse};
use serde::Serialize;

use crate::Respondable;

#[derive(Clone)]
struct UrlEncoded<T>(T);

impl<S: Serialize> Respondable for UrlEncoded<S> {
  fn respond(self, _req: &HttpRequest) -> HttpResponse {
    let body = serde_urlencoded::to_string(&self.0).unwrap();
    let mut res = HttpResponse::new(body);
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_WWW_FORM_URLENCODED.to_string().parse().unwrap(),
    );
    res
  }
}

// impl<T> FromRequest for UrlEncoded<T>
// where
//   T: DeserializeOwned + 'static + Clone, // 'static maybe brings problems
// {
//   type Error = BodyParseError;
//   type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
//   fn from_request(req: &HttpRequest) -> Self::Future {
//     let content_type = req.headers().get("Content-Type");
//     let output = {
//       let body = req.body();
//       if content_type.is_none()
//         || content_type.is_some_and(|v| v != "application/x-www-form-urlencoded")
//       {
//         Err(BodyParseError::ContentTypeInvalid)
//       } else if body.is_empty() {
//         Err(BodyParseError::NoBody)
//       } else {
//         let json = serde_urlencoded::from_str(body).unwrap();
//         Ok(UrlEncoded(json))
//       }
//     };
//
//     Box::pin(async move { output })
//   }
// }
