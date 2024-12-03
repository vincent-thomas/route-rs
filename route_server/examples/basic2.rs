use std::error::Error;

use route::{
  endpoint::Endpoint,
  http::{request::Head, StatusCode},
  respond::Respondable,
  App,
};
use route_server::ServerBuilder;
use tokio::task;

struct ThisGuard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new();

  app.at("/testing", Endpoint::testing("testing".to_string()));
  ServerBuilder::bind("127.0.0.1", 3000).app(app).run().await
}
