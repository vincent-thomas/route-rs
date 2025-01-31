use std::time::Duration;

use futures_util::{stream, StreamExt as _};
use titan::{
  http::{Request, Respondable},
  web::{self, Event, Sse},
  App,
};
use tokio::{net::TcpListener, time::sleep};

async fn test(_: Request) -> impl Respondable {
  let chunks = vec![Event::new("yes".to_string()); 5];

  let nice = stream::iter(chunks)
    .map(|item| {
      let delay = sleep(Duration::from_secs(1));
      async move {
        delay.await; // Delay for 1 second
        item
      }
    })
    .buffer_unordered(1);
  Sse(nice)
}

#[tokio::main]
async fn main() {
  let app = App::default().at("/", web::get(test));

  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  titan::serve(listener, app).await.unwrap();
}
