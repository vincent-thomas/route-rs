#![allow(dead_code, unused_variables, unused_imports)]

use titan::{lambda, web, App};

async fn testing() -> String {
  "hello world".to_string()
}

#[tokio::main]
async fn main() {
  let app = App::default()
    .at("/", web::Redirect::permanent("/testing"))
    .at("/testing", web::get(testing));

  // 1. lambda::run(app).await.unwrap()
  // 2. lambda::run(lambda::wrap_handler(testing)).await.unwrap()
  //
  // Both works.
}
