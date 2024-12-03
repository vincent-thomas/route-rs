use std::error::Error;

use route::{
  http::{request::Head, StatusCode},
  resource::{Guard, GuardOutcome, GuardReason},
  web::{self, Json, LongPollingResource, SSEResource},
  App, Respondable,
};
use route_server::ServerBuilder;
use tokio::task;

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
  (StatusCode::NO_CONTENT, body)
}

async fn handler2() -> String {
  "Hello, world!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new();

  let (sse_resource, sender) = SSEResource::<Json<i32>>::new();
  let (lpresource, lp_sender) = LongPollingResource::<Json<i32>>::new();
  app.at("/sse", sse_resource);
  app.at("/lp", lpresource);
  app.at(
    "/api",
    web::resource()
      .route(web::route().guard(ThisGuard).get(handler).post(handler2)),
  );

  app.at("/api2", web::resource().route(web::route().get(handler)));
  app.at("/redirect", web::redirect("https://google.com"));

  task::spawn(async move {
    loop {
      tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
      let _ = lp_sender.send(Json(10));
    }
  });
  task::spawn(async move {
    let mut start = 0;
    loop {
      let _ = sender.send(web::Json(start));
      start += 1;
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
  });

  ServerBuilder::bind("127.0.0.1", 3000).app(app).run().await
}
