use route::{
  guard::{Guard, GuardOutcome},
  http::request::Parts,
  server::Server,
  web::{self, authorization::BearerAuthorization, Cookies, Json},
  App, Respondable,
};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;

struct AuthGuard;

impl Guard for AuthGuard {
  fn check(&self, _: &Parts) -> GuardOutcome {
    println!("Checking");
    GuardOutcome::WeJustPassinBy
  }
}

#[derive(Deserialize, Debug)]
struct Thing {
  nice: String,
}

async fn test(
  Cookies(cookies): Cookies,
  BearerAuthorization(token): BearerAuthorization,
  Json(body): Json<Thing>,
) -> impl Respondable {
  Json(json!({
      "test": body.nice,
      "value": token,
      "nice_cookie": cookies.get("nice").unwrap()
  }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();
  app.at("/testing/{id}", web::with_guard(AuthGuard, web::post(test)));

  Server::bind("127.0.0.1", 3000).run(app).await
}
