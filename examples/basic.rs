use route::{Address, App, Service};
use route_contract::{method::HttpMethod, variable::ExtractVariable};

struct MyService;

impl Service for MyService {
  fn method(&self) -> HttpMethod {
    HttpMethod::Get
  }

  fn path(&self) -> String {
    "/user/{user_id}/test".to_string()
  }
}

#[tokio::main]
async fn main() {
  let mut app = App::default();

  app.service(MyService, |req| {
    println!("Req: {req:#?}");
    let test: String = req.variables.get("user_id").unwrap().clone().extract().unwrap();
    test
  });

  app.bind(Address(127, 0, 0, 1)).listen(8080).await;
}
