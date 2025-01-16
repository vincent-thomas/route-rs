use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use titan::{
  web::{self, authorization, Cookies, Json, Query},
  App, Respondable,
};
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

use titan_derive::ssg;

#[titan::ssg]
pub fn testing() -> titan_html::tags::html::Html {
  titan::html::tags::html::Html::from((
    titan::html::tags::head::Head::default(),
    titan::html::tags::Body::default(),
  ))
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
  titan::serve(listener, app).await
}
