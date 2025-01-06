use route::{web, App, Respondable};
use std::{io, time::Duration};
use tokio::net::TcpListener;

async fn index() -> impl Respondable {
  tokio::time::sleep(Duration::from_secs(2)).await;
  "OK"
}

async fn protected() -> impl Respondable {
  "OK"
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let app = App::default()
    .at("/", web::get(index))
    .at("/admin", web::get(protected))
    .at("/redirect", web::Redirect::new(true, "/admin"));

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  route::serve(listener, app).await
}
