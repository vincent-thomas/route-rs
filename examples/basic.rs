use route::{
  http::request::HttpRequest,
  types::{Bytes, Cookie, Json, UrlEncoded},
  App, FromRequest, FromRequestParts, Respondable,
};
use route_http::request::Head;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Test {
  name: String,
  annan_sak: i32,
}

async fn test(req: HttpRequest) -> impl Respondable {
  //let test = Cookie::from_request(nice_request).await;
  let test = Test { name: "test".to_string(), annan_sak: 43 };
  UrlEncoded(test)
}

async fn test2(_req: HttpRequest) -> impl Respondable {
  let test = move |test: Cookie, req: Json<Test>| {
    let test = Test { name: "fdashjkfdlas".to_string(), annan_sak: 32 };
    Json(test)
  };

  let (head, body) = _req.clone().into_parts();

  test(Cookie::extract_parts(&head as &Head).await.unwrap(), Json::extract(_req).await.unwrap())
}

async fn test3(_req: HttpRequest) -> impl Respondable {
  Bytes("test2".as_bytes().to_vec())
}

#[tokio::main]
async fn main() {
  let mut app = App::new();

  app.at("/test").get(test).post(test).put(test3);
  app.at("/another").get(test);

  app.bind((127, 0, 0, 1).into()).listen(3000).await;
}
