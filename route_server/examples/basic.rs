use std::error::Error;

use route::{
  http::request::Head,
  http::StatusCode,
  resource::GuardReason,
  resource::{Guard, GuardOutcome},
  web, App, Respondable,
};
use route_server::ServerBuilder;

struct ThisGuard;

impl Guard for ThisGuard {
  fn check(&self, head: &Head) -> GuardOutcome {
    if head.headers.get("Test").is_some() {
      GuardOutcome::WeJustPassinBy
    } else {
      GuardOutcome::Reason(GuardReason::Forbidden)
    }
  }
}

async fn handler(body: web::Bytes) -> impl Respondable {
  dbg!(&body);

  (StatusCode::NO_CONTENT, body)
}

async fn handler2() -> String {
  "Hello, world!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new();

  app.at(
    "/api",
    web::resource()
      .route(web::route().guard(ThisGuard).get(handler).post(handler2)),
  );
  app.at("/api2", web::resource().route(web::route().get(handler)));

  app.at("/redirect", web::redirect("https://google.com"));

  ServerBuilder::bind("127.0.0.1", 3000).app(app).run().await
}
