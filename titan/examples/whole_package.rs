use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use titan::{
  web::{self, Cookies, Json, Params, Query},
  App, Respondable,
};
use tokio::net::TcpListener;

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
  let app = App::default().at("/:test/nice", web::get(test));

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  titan::serve(listener, app).await.unwrap();
  Ok(())
}
