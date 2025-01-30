use titan::{lambda, web::Redirect, App};

#[tokio::main]
async fn main() {
  let app = App::default().at("/", Redirect::permanent("/nice"));
  lambda::run(app).await.unwrap()
}
