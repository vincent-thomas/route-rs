use std::fmt::Debug;

use serde::Serialize;

use super::HttpResponse;

pub trait Respondable
where
  Self: Clone + Debug + Send + Sync,
{
  fn respond(self) -> HttpResponse;
}

impl<T> Respondable for T
where
  T: Serialize + Send + Sync + Clone + Debug,
{
  fn respond(self) -> HttpResponse {
    let body = serde_json::to_vec(&self).unwrap();
    HttpResponse::Ok().body_bytes(body)
  }
}
