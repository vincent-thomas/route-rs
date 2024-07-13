use route::{
  types::{bytes::Bytes, json::Json, urlencoded::UrlEncoded},
  App, Respondable2,
};
use route_http::request::HttpRequest2;
use serde::Serialize;

#[derive(Serialize)]
struct Test {
  name: String,
  annan_sak: i32,
}

async fn test(_req: HttpRequest2) -> impl Respondable2 {
  let test = Test { name: "test".to_string(), annan_sak: 43 };
  UrlEncoded(test)
}

async fn test2(_req: HttpRequest2) -> impl Respondable2 {
  let test = Test { name: "fdashjkfdlas".to_string(), annan_sak: 32 };
  Json(test)
}

async fn test3(_req: HttpRequest2) -> impl Respondable2 {
  Bytes("test2".as_bytes().to_vec())
}

#[tokio::main]
async fn main() {
  let mut app = App::new();

  app.at("/test").get(test2).post(test).put(test3);
  app.at("/another").get(test);

  app.bind((127, 0, 0, 1).into()).listen(3000).await;
}
