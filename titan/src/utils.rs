use std::task::{Context, Poll};

use futures_util::future::BoxFuture;
use titan_core::{Service, ServiceExt};

// Required for titan-derive
pub use lazy_static::lazy_static;

pub use futures_util::FutureExt;

pub trait CloneService<R>: Service<R> {
  fn clone_box(
    &self,
  ) -> Box<
    dyn CloneService<
        R,
        Response = Self::Response,
        Error = Self::Error,
        Future = Self::Future,
      > + Send
      + Sync,
  >;
}
impl<R, T> CloneService<R> for T
where
  T: Service<R> + Send + Sync + Clone + 'static,
{
  fn clone_box(
    &self,
  ) -> Box<
    dyn CloneService<
        R,
        Response = T::Response,
        Error = T::Error,
        Future = T::Future,
      > + Send
      + Sync,
  > {
    Box::new(self.clone())
  }
}
impl<T, U, E> Clone for BoxCloneService<T, U, E> {
  fn clone(&self) -> Self {
    Self(self.0.clone_box())
  }
}

pub struct BoxCloneService<T, U, E>(
  Box<
    dyn CloneService<
        T,
        Response = U,
        Error = E,
        Future = BoxFuture<'static, Result<U, E>>,
      > + Send
      + Sync,
  >,
);

impl<T, U, E> BoxCloneService<T, U, E> {
  pub(crate) fn new<S>(inner: S) -> Self
  where
    S: Service<T, Response = U, Error = E> + Clone + Send + Sync + 'static,
    S::Future: Send + 'static,
  {
    let inner = inner.map_future(|f| Box::pin(f) as _);
    BoxCloneService(Box::new(inner))
  }
}

impl<T, U, E> Service<T> for BoxCloneService<T, U, E> {
  type Response = U;
  type Error = E;
  type Future = BoxFuture<'static, Result<U, E>>;

  #[inline]
  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>> {
    self.0.poll_ready(cx)
  }

  #[inline]
  fn call(&mut self, request: T) -> Self::Future {
    self.0.call(request)
  }
}
