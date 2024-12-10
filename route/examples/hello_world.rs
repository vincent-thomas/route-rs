use std::io;

use route::web;
use tokio::net::TcpListener;

async fn test() -> &'static str {
  "Hello World"
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

  route::serve(listener, web::any(test)).await
}
