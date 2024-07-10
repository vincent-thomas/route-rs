use route::{endpoint::EndpointRouter, App};
use route_http::request::HttpRequest;

async fn test(req: HttpRequest) -> String {
  "test".to_string()
}

fn main() {
  let mut app = App::default();

  let endpoint = EndpointRouter::default().get(test);

  app.service("/test", endpoint);
}

// struct MyService;
//
// impl Service for MyService {
//   fn method(&self) -> HttpMethod {
//     HttpMethod::Get
//   }
//
//   fn path(&self) -> String {
//     "/user/{user_id}".to_string()
//   }
//   fn handler(self, req: HttpRequest) -> impl route_http::response::Respondable {
//     let test: String = req.variables.get("user_id").unwrap().clone().extract().unwrap();
//     test
//   }
//
//   // fn handler(&self, req: HttpRequest) -> impl route_http::respondable::Respondable {
//   //   let test: String = req.variables.get("user_id").unwrap().clone().extract().unwrap();
//   //   test
//   // }
// }
//
// // #[route::get("/user/{user_id}")]
// // fn get_user(req: HttpRequest) -> String {
// //   println!("{req:?}");
// //   "".to_string()
// // }
// #[tokio::main]
// async fn main() {
//   let mut app = App::new();
//
//   app.service(MyService, |req| {
//     println!("Req: {req:#?}");
//     let test: String = req.variables.get("user_id").unwrap().clone().extract().unwrap();
//     test
//   });
//   app.service(MyService, |req| 4);
//   app.bind(Address(127, 0, 0, 1)).listen(8080).await;
// }
