use std::future::Future;

pub trait Handler<Args>: Send + 'static {
  type Output;
  type Future: Future<Output = Self::Output>;
  fn call(&self, req: Args) -> Self::Future;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Send + 'static,
        Fut: Future + Send,
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
        }
    }
});
impl<Func, Fut> Handler<()> for Func
where
  Func: Fn() -> Fut + Send + 'static,
  Fut: Future + Send,
{
  type Future = Fut;
  type Output = Fut::Output;

  fn call(&self, req: ()) -> Self::Future {
    let _ = req;
    self()
  }
}

factory_tuple! { A }
factory_tuple! { A B }
factory_tuple! { A B C }
factory_tuple! { A B C D }
factory_tuple! { A B C D E }
factory_tuple! { A B C D E F }
factory_tuple! { A B C D E F I }
factory_tuple! { A B C D E F I J }
factory_tuple! { A B C D E F I J K }
factory_tuple! { A B C D E F I J K L }
