// use std::{collections::HashMap, future::Future, pin::Pin};

// use route_http::{method::Method, request::HttpRequest, response::HttpResponse};

// use crate::handler::Handler;

// #[derive(Clone)]
// pub struct EndpointRouter<R = ()> {
//   endpoints: HashMap<Method, R>,
//   // get: Option<R>,
//   // post: Option<R>,
//   // put: Option<R>,
//   // patch: Option<R>,
//   // delete: Option<R>,
// }

// impl<T> Handler for Route<T>
// where
//   T: Handler,
// {
//   type Future = T::Future;
//   fn call(self, req: HttpRequest) -> Self::Future {
//     self.0.call(req)
//   }
// }

// #[derive(Clone)]
// struct Route<T>(T);

// impl<R> Default for EndpointRouter<R> {
//   fn default() -> Self {
//     EndpointRouter { endpoints: HashMap::new() }
//     // EndpointRouter { get: None, post: None, put: None, patch: None, delete: None }
//   }
// }

// impl<R> EndpointRouter<R>
// where
//   R: Clone + Handler,
// {
//   pub fn on(self, method: Method, handler: R) -> EndpointRouter<Route<R>>
// // R: Send + Sync + 'static,
//   {
//     // fn set_on_method<H>(endpoint: &mut EndpointRouter<H>, method: Method, handler: H)
//     // where
//     //   H: Handler,
//     // {
//     if let Some(_shouldnt_exist) = self.endpoints.get(&method) {
//       panic!("Method already exists");
//     } else {
//       let mut this_clone = self.clone();
//       this_clone.endpoints.insert(method, Route(handler));
//       this_clone
//       // self.endpoints.insert(method, Route(handler));
//     }
//     //
//     // set_on_method(&mut self, filter, handler);

//     // EndpointRouter::default().on(filter, handler)
//     self
//   }
// }

// macro_rules! add_method {
//   ($name:ident) => {
//     pub fn $name<R>(handler: R) -> EndpointRouter<R>
//     where
//       R: $crate::handler::Handler + 'static + Clone,
//     {
//       // let router = EndpointRouter::default();

//       on(paste::paste! {Method::[<$name:upper>]}, handler)
//       // self.$name = Some(handler);
//       // self
//     }
//   };
// }

// add_method!(get);
// add_method!(post);
// add_method!(delete);

// pub fn on<H>(method: Method, handler: H) -> EndpointRouter<H>
// where
//   H: Handler + Clone,
// {
//   EndpointRouter::default().on(method, handler)
//   // fn set_on_method<H>(endpoint: &mut EndpointRouter<H>, method: Method, handler: H)
//   // where
//   //   H: Handler,
//   // {
//   // if let Some(_shouldnt_exist) = self.endpoints.get(&method) {
//   //   panic!("Method already exists");
//   // } else {
//   //   self.endpoints.insert(method, Box::new(handler));
//   // }
//   // //
//   // // set_on_method(&mut self, filter, handler);
//   //
//   // // EndpointRouter::default().on(filter, handler)
//   // self
// }

// // pub fn on<H, T, S>(filter: MethodFilter, handler: H) -> MethodRouter<S, Infallible>
// // where
// //     H: Handler<T, S>,
// //     T: 'static,
// //     S: Clone + Send + Sync + 'static,
// // {
// //     MethodRouter::new().on(filter, handler)
// // }
// //

// // impl<R> EndpointRouter<R>
// // where
// //   R: Clone + Send + Sync + 'static,
// // {
// //   add_method!(get);
// //   add_method!(post);
// //   add_method!(put);
// //   add_method!(patch);
// //   add_method!(delete);
// // }
// //
// //

// macro_rules! check_method_branch {
//   ($method:ident, $req:ident, $self:ident) => {
//     if $req.method() == Method::$method {
//       return EndpointRouter::check_and_if_there_run(
//         $self.endpoints.get(&Method::$method).cloned(),
//         $req,
//       )
//       .await;
//     }
//   };
// }
// impl<R> Handler for EndpointRouter<R>
// where
//   R: Handler + Clone,
// {
//   type Future = Pin<Box<dyn Future<Output = HttpResponse>>>;
//   // type Future = Pin<Box<dyn Future<Output = HttpResponse>>>;
//   fn call(self, _req: HttpRequest) -> Self::Future {
//     let output = async move {
//       /* if _req.method() == Method::GET {
//         EndpointRouter::check_and_if_there_run(self.endpoints.get(&Method::GET).cloned(), _req)
//           .await
//       } else if _req.method() == Method::POST {
//         EndpointRouter::check_and_if_there_run(self.post, _req).await
//       } else if _req.method() == Method::PUT {
//         EndpointRouter::check_and_if_there_run(self.put, _req).await
//       } else if _req.method() == Method::PATCH {
//         EndpointRouter::check_and_if_there_run(self.patch, _req).await
//       } else if _req.method() == Method::DELETE {
//         EndpointRouter::check_and_if_there_run(self.delete, _req).await
//       } else if _req.method() == Method::HEAD {
//         EndpointRouter::fix_head(self, _req).await
//       } */
//       check_method_branch!(GET, _req, self);
//       check_method_branch!(POST, _req, self);
//       check_method_branch!(PUT, _req, self);
//       check_method_branch!(PATCH, _req, self);
//       check_method_branch!(DELETE, _req, self);
//       if _req.method() == Method::HEAD {
//         return EndpointRouter::fix_head(self, _req).await;
//       }
//       HttpResponse::new("".to_string())
//     };
//     Box::pin(output)
//   }
// }

// impl<R> EndpointRouter<R>
// where
//   R: Handler + Clone,
// {
//   async fn check_and_if_there_run(
//     real: Option<R>,
//     req: route_http::request::HttpRequest,
//   ) -> HttpResponse {
//     match real {
//       Some(nice) => nice.call(req).await,
//       None => {
//         HttpResponse::new("".to_string())
//         // HttpResponse::MethodNotAllowed().content_type("text/plain").body("Method not allowed")
//       }
//     }
//   }

//   async fn fix_head(self, req: route_http::request::HttpRequest) -> HttpResponse {
//     let exists = self.endpoints.get(&Method::GET).cloned();

//     match exists {
//       Some(nice) => {
//         let mut output = nice.call(req).await;
//         let body_mut = output.body_mut();
//         body_mut.clear();

//         output
//       }
//       None => {
//         let mut res = HttpResponse::new("".to_string());
//         *res.status_mut() = route_http::status::StatusCode::METHOD_NOT_ALLOWED;
//         res
//       }
//     }
//   }
// }
