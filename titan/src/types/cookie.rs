use std::collections::HashMap;

use titan_core::FromRequestParts;
use titan_http::{header::HeaderValue, Parts};

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
      let item: Vec<&str> =
        item.trim().split("=").filter(|v| !v.is_empty()).collect();

      let key = match item.first() {
        Some(v) => v,
        None => continue,
      };

      let value = match item.get(1) {
        Some(v) => v,
        None => continue,
      };

      vec.push((key.to_string(), value.to_string()))
    }

    let hash = HashMap::from_iter(vec);

    Ok(Self(hash))
  }
}
