use std::{error::Error, io::Write};

use route_http::{request::Request, response::Response};

#[async_trait::async_trait]
pub trait Service: Sync + Send {
  async fn call_service(&self, req: Request) -> Response<Box<[u8]>>;
}
