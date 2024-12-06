use route::{
  guard::{Guard, GuardOutcome, GuardReason},
  http::request::Parts,
  server::Server,
  web, App, Respondable,
};
use std::error::Error;

struct AuthGuard;

impl Guard for AuthGuard {
  fn check(&self, _: &Parts) -> GuardOutcome {
    GuardOutcome::Reason(GuardReason::Unauthorized)
  }
}

async fn index() -> impl Respondable {
  "OK"
}

async fn protected() -> impl Respondable {
  "OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();

  app.at("/", web::get(index));
  app.at("/admin", web::with_guard(AuthGuard, web::get(protected)));

  Server::bind("127.0.0.1", 3000).run(app).await
}
