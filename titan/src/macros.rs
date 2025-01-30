#[macro_export]
macro_rules! all_the_tuples {
  ($name:ident) => {
    $name!([], T1);
    $name!([T1], T2);
    $name!([T1, T2], T3);
    $name!([T1, T2, T3], T4);
    $name!([T1, T2, T3, T4], T5);
    $name!([T1, T2, T3, T4, T5], T6);
    $name!([T1, T2, T3, T4, T5, T6], T7);
    $name!([T1, T2, T3, T4, T5, T6, T7], T8);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
      T16
    );
  };
}

#[macro_export]
macro_rules! always_ready {
  () => {
    fn poll_ready(
      &mut self,
      _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
      std::task::Poll::Ready(Ok(()))
    }
  };
}
