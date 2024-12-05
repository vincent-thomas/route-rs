use std::collections::HashMap;

use route_core::FromRequestParts;
use route_http::{header::HeaderValue, request::Parts};

pub struct Cookies(pub HashMap<String, String>);

impl FromRequestParts for Cookies {
  type Error = ();
  fn from_request_parts(parts: &mut Parts) -> Result<Self, Self::Error> {
    let default = HeaderValue::from_static("");
    let cookie_iter = parts
      .headers
      .get("cookie")
      .unwrap_or(&default)
      .to_str()
      .unwrap()
      .split(";");

    let mut vec = Vec::new();

    for item in cookie_iter {
      let item: Vec<&str> = item.trim().split("=").collect();

      let key = item[0];
      let value = item[1];

      vec.push((key.to_string(), value.to_string()))
    }

    let hash = HashMap::from_iter(vec);

    Ok(Self(hash))
  }
}
