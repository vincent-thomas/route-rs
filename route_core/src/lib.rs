pub mod request;
// pub trait Handler: Clone + Send + Sized + 'static {
//   // type Future: Future<Output = HttpResponse> + Send + 'static;
//
//   fn call(self, req: HttpRequest) -> Box<dyn Future<Output = HttpResponse> + Send + 'static>;
// }
