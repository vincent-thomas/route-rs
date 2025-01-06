use std::{
  future::Future,
  io,
  pin::Pin,
  task::{Context, Poll},
};

use route::Service;
use route_server::IncomingStream;
use tokio::net::TcpListener;

#[derive(Clone)]
struct App;

impl Service<IncomingStream<'_>> for App {
  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
  type Error = ();
  type Response = ();

  fn poll_ready(
    &mut self,
    _: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: IncomingStream<'_>) -> Self::Future {
    println!("här kör vi");

    Box::pin(async move { Ok(()) })
  }
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

  let app = App;

  route::serve(listener, app).await
}
