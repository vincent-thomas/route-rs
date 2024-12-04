use std::error::Error;

use route::{endpoint::Endpoint, App};
use route_server::Server;

async fn test(testing: String, testing2: String) -> usize {
  println!("{testing} {testing2}");
  5
}

async fn test2(testing: String) -> String {
  println!("{testing}");
  "".into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();

  app.at("/testing", Endpoint::default().get(test));
  app.at("/testing/{id}", Endpoint::default().get(test2).post(test));

  Server::bind("127.0.0.1", 3000).run(app.service).await
}
