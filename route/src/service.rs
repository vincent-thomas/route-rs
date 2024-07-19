use std::{future::Future, pin::Pin};

use route_http::{request::HttpRequest, response::HttpResponse};

// / # HttpService
// / Called with a request and return a response.
// / Anything that implements this trait can be used as a handler for a route.
// /
// #[async_trait::async_trait]
// pub trait HttpService {
//   fn call_service(
//     &'static self,
//     req: HttpRequest,
//   ) -> Pin<Box<dyn Future<Output = HttpResponse> + 'static>>;
// }

// impl<Args, H> HttpService for H
// where
//   H: Handler<Args>,
//   Args: FromRequest + Send,
//   H::Output: Respondable,
// {
//   async fn call_service(&self, req: HttpRequest) -> HttpResponse {
//     let args = match Args::from_request(req).await {
//       Ok(args) => args,
//       Err(resp) => {
//         let error: Error = resp.into();
//         let mut res = HttpResponse::new(error.message.clone());
//         *res.status_mut() = error.checked_method().clone();
//         return res;
//       }
//     };
//     let output = self.call(args).await;
//     output.respond()
//   }
// }
