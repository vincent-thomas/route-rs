use route::{web, App, Respondable};
use route_server::Server;
use std::error::Error;

async fn test() -> impl Respondable {
  "Hello World"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::default();
  app.at("/", web::get(test));

  Server::bind("127.0.0.1", 3000).run(app).await
}
