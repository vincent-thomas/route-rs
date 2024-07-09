use std::future::Future;

use route_http::{response::HttpResponse, HttpRequest};
pub trait Handler: Clone + Send + Sized + 'static {
  type Future: Future<Output = HttpResponse> + Send + 'static;

  // Required method
  fn call(self, req: HttpRequest) -> Self::Future;
}

// pub trait Handler<Args> {
//   type Output;
//   type Future: Future<Output = Self::Output>;

//   fn call(&self, args: Args) -> Self::Future;
// }

// macro_rules! factory_tuple ({ $($param:ident)* } => {
//     impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
//     where
//         Func: Fn($($param),*) -> Fut + Clone + 'static,
//         Fut: Future,
//     {
//         type Output = Fut::Output;
//         type Future = Fut;

//         #[inline]
//         #[allow(non_snake_case)]
//         fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
//             (self)($($param,)*)
//         }
//     }
// });

// factory_tuple! {}
// factory_tuple! { A }
// factory_tuple! { A B }
// factory_tuple! { A B C }
// factory_tuple! { A B C D }
// factory_tuple! { A B C D E }
// factory_tuple! { A B C D E F }
// factory_tuple! { A B C D E F G }
// factory_tuple! { A B C D E F G H }
// factory_tuple! { A B C D E F G H I }
// factory_tuple! { A B C D E F G H I J }
// factory_tuple! { A B C D E F G H I J K }
// factory_tuple! { A B C D E F G H I J K L }
// factory_tuple! { A B C D E F G H I J K L M }
// factory_tuple! { A B C D E F G H I J K L M N }
// factory_tuple! { A B C D E F G H I J K L M N O }
// factory_tuple! { A B C D E F G H I J K L M N O P }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[tokio::test]
//   async fn test_handler() {
//     let handler = |x: i32, r: i32| async move { x + 1 };
//     let h = handler.clone();
//     let h = h.call(1).await;
//     assert_eq!(h, 2);
//   }
// }
