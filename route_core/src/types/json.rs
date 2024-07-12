use route_http::{request::HttpRequest, response::HttpResponse};
use serde::Serialize;

use crate::Respondable;

struct Json<T>(T);

// impl<T> FromRequest for Json<T>
// where
//   T: DeserializeOwned + 'static, // 'static maybe brings problems
// {
//   type Error = BodyParseError;
//   type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
//   fn from_request(req: &HttpRequest) -> Self::Future {
//     let content_type = req.headers().get("Content-Type");
//     let output = {
//       let body = req.body();
//       if content_type.is_none() || content_type.is_some_and(|v| v != "application/json") {
//         Err(BodyParseError::ContentTypeInvalid)
//       } else if body.is_empty() {
//         Err(BodyParseError::NoBody)
//       } else {
//         let json = serde_json::from_str(body).unwrap();
//         Ok(Json(json))
//       }
//     };
//
//     Box::pin(async move { output })
//   }
// }

impl<S: Serialize> Respondable for Json<S> {
  fn respond(self, _req: &HttpRequest) -> HttpResponse {
    let body = serde_json::to_string(&self.0).unwrap();
    let mut res = HttpResponse::new(body);
    let headers = res.headers_mut();

    headers.insert(
      route_http::header::CONTENT_TYPE,
      route_http::mime::APPLICATION_JSON.to_string().parse().unwrap(),
    );
    res

    // HttpResponse::new(200, serde_json::to_string(&self.0).unwrap())
  }
}
