use std::{collections::HashMap, future::Future, pin::Pin};

use route_core::FromRequest;
// use route_core::FromRequestParts;
use route_http::request::HttpRequest;

use super::BodyParseError;

pub struct Cookie(pub HashMap<String, String>);

impl FromRequest for Cookie {
  type Error = BodyParseError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
  fn from_request(req: HttpRequest) -> Self::Future {
    let cookie_builder = HashMap::new();

    let test = req.headers().get("cookie");

    dbg!(test);

    Box::pin(async move { Ok(Cookie(cookie_builder)) })

    //unimplemented!()
  }
}
