use std::collections::HashMap;

use route_core::FromRequest;
// use route_core::FromRequestParts;
use route_http::request::HttpRequest;

use super::BodyParseError;

pub struct Cookie(pub HashMap<String, String>);

impl FromRequest for Cookie {
  type Error = BodyParseError;
  fn from_request(_req: HttpRequest) -> Result<Self, Self::Error> {
    //let cookie_builder = HashMap::new();

    // let test = req.headers().get("cookie");

    //Ok(Cookie(cookie_builder));
    todo!();

    //unimplemented!()
  }
}
