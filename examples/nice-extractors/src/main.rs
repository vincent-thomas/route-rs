use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use titan::{
  html::tags::{head::Head, html::Html, Body},
  http::Respondable,
  web::{self, authorization, Cookies, Json, Query},
  App,
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

#[titan::ssg]
async fn testing() -> Html {
  Html::from((Head::default(), Body::default()))
}

async fn index(
  Cookies(cookies): Cookies,
  authorization::Bearer(token): authorization::Bearer,
  Query(queries): Query<Queries>,
  Json(body): Json<Thing>,
) -> impl Respondable {
  Json(json!({
      "body": body,
      "nice_cookie": cookies,
      "auth": token,
      "queries": queries
  }))
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let app = App::default().at("/", web::post(index));

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  titan::serve(listener, app).await // Or titan::serve(listener, web::post(index)).await for any path, web::any for any method
}
