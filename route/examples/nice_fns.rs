use route::{
  web::{self, authorization, Cookies, Json, Query},
  App, Respondable,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use tokio::net::TcpListener;

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
async fn main() -> io::Result<()> {
  let app = App::default().at("/", web::post(index));

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  route::serve(listener, app).await
}
