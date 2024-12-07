use route::{
  guard::{Guard, GuardOutcome},
  http::request::Parts,
  server::Server,
  web::{self, Cookies, Json, Params, Query},
  App, Respondable,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

struct AuthGuard;

impl Guard for AuthGuard {
  fn check(&self, _: &Parts) -> GuardOutcome {
    GuardOutcome::WeJustPassinBy
  }
}

#[derive(Deserialize, Debug, Serialize)]
struct Thing {
  nice: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Queries {
  nice: String,
  test: Option<i32>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Param {
  test: String,
}

async fn test(
  Cookies(cookies): Cookies,
  Query(queries): Query<Queries>,
  Params(params): Params<Param>,
  Json(body): Json<Thing>,
) -> impl Respondable {
  Json(json!({
      "body": body,
      "nice_cookie": cookies.get("nice"),
      //"auth": token,
      "queries": queries,
      "param": params
  }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();
  app.at("/:test/nice", web::with_guard(AuthGuard, web::post(test)));

  Server::bind("127.0.0.1", 3000).run(app).await
}
