use route::{endpoint::post, App, Respondable};
use route_http::request::HttpRequest;

struct Test {
  name: String,
}

async fn test(req: HttpRequest) -> impl Respondable {
  let test = Test { name: "test".to_string() };
  // test.from_request(req);
  "".to_string()
}

async fn test2(req: HttpRequest) -> String {
  let test = Test { name: "test".to_string() };
  // test.from_request(req);
  3
}

#[tokio::main]
async fn main() {
  let mut app = App::new();

  let endpoint = post(test);
  // let endpoint2 = EndpointRouter::default().get(test2);
  app.service("/test", endpoint);
  app.service("/test", get(test2));
  // app.service("/test2", endpoint2);

  // app.service("/test", endpoint);
  // app.bind(route::address::Address(0, 0, 0, 0)).listen(8080).await;
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
