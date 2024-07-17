use route::{
  resource::{Guard, GuardOutcome},
  web, App,
};
use route_http::request::Head;
use route_server::ServerBuilder;

struct ThisGuard;

impl Guard for ThisGuard {
  fn check(&self, _head: &Head) -> GuardOutcome {
    GuardOutcome::WeJustPassinBy
  }
}

async fn handler() -> &'static str {
  "Hello, world!"
}

async fn handler2() -> String {
  "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
  let mut app = App::new();

  app.at(
    "/api",
    web::resource()
      .guard(ThisGuard)
      .route(web::route().guard(ThisGuard).get(handler).post(handler2)),
  );
  app.at("/api2", web::resource().route(web::route().get(handler)));

  let server = ServerBuilder::bind("127.0.0.1", 3000).app(app);

  let _ = server.run().await;
}
