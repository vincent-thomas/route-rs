use route::{
  types::{Bytes, Json, UrlEncoded},
  App, Respondable,
};
use route_http::request::HttpRequest;
use serde::Serialize;

#[derive(Serialize)]
struct Test {
  name: String,
  annan_sak: i32,
}

async fn test(_req: HttpRequest) -> impl Respondable {
  let test = Test { name: "test".to_string(), annan_sak: 43 };
  UrlEncoded(test)
}

async fn test2(_req: HttpRequest) -> impl Respondable {
  let test = Test { name: "fdashjkfdlas".to_string(), annan_sak: 32 };
  Json(test)
}

async fn test3(_req: HttpRequest) -> impl Respondable {
  Bytes("test2".as_bytes().to_vec())
}

#[tokio::main]
async fn main() {
  let mut app = App::new();

  app.at("/test").get(test2).post(test).put(test3);
  app.at("/another").get(test);

  app.bind((127, 0, 0, 1).into()).listen(3000).await;
}
