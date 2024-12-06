use route::{
  server::Server,
  web::{self, authorization, Cookies, Json, Query},
  App, Respondable,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Deserialize, Debug, Serialize)]
struct Thing {
  nice: String,
  test: i32,
}

#[derive(Deserialize, Debug, Serialize)]
struct Queries {
  nice: String,
  test: Option<i32>,
}

async fn index(
  Cookies(cookies): Cookies,
  authorization::Bearer(token): authorization::Bearer,
  Query(queries): Query<Queries>,
  Json(body): Json<Thing>,
) -> impl Respondable {
  Json(json!({
      "body": body,
      "nice_cookie": cookies.get("nice").unwrap(),
      "auth": token,
      "queries": queries
  }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();
  app.at("/", web::post(index));

  Server::bind("127.0.0.1", 3000).run(app).await
}
