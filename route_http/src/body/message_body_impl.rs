use std::convert::Infallible;
use std::{
  mem,
  ops::DerefMut,
  pin::Pin,
  task::{Context, Poll},
};

use bytes::Bytes;

use super::{BodySize, MessageBody};

impl<B> MessageBody for &mut B
where
  B: MessageBody + Unpin + ?Sized,
{
  type Error = B::Error;

  fn size(&self) -> BodySize {
    (**self).size()
  }

  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    Pin::new(&mut **self).poll_next(cx)
  }
}

impl<B> MessageBody for Box<B>
where
  B: MessageBody + Unpin + ?Sized,
{
  type Error = B::Error;
  fn size(&self) -> BodySize {
    self.as_ref().size()
  }

  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    Pin::new(self.get_mut().as_mut()).poll_next(cx)
  }
}

impl<T, B> MessageBody for Pin<T>
where
  T: DerefMut<Target = B> + Unpin,
  B: MessageBody + ?Sized,
{
  type Error = B::Error;

  fn size(&self) -> BodySize {
    self.as_ref().size()
  }

  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    self.get_mut().as_mut().poll_next(cx)
  }
}

impl MessageBody for () {
  type Error = Infallible;
  fn size(&self) -> BodySize {
    BodySize::Sized(0)
  }

  fn poll_next(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    Poll::Ready(None)
  }
}

impl MessageBody for Bytes {
  type Error = Infallible;
  fn size(&self) -> BodySize {
    BodySize::Sized(self.len() as u64)
  }

  fn poll_next(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    if self.is_empty() {
      Poll::Ready(None)
    } else {
      Poll::Ready(Some(Ok(mem::take(self.get_mut()))))
    }
  }
}

impl MessageBody for Vec<u8> {
  type Error = Infallible;

  fn size(&self) -> BodySize {
    BodySize::Sized(self.len() as u64)
  }

  fn poll_next(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    if self.is_empty() {
      Poll::Ready(None)
    } else {
      Poll::Ready(Some(Ok(mem::take(self.get_mut()).into())))
    }
  }
}

impl MessageBody for &'static str {
  type Error = Infallible;

  #[inline]
  fn size(&self) -> BodySize {
    BodySize::Sized(self.len() as u64)
  }

  #[inline]
  fn poll_next(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    if self.is_empty() {
      Poll::Ready(None)
    } else {
      let string = mem::take(self.get_mut());
      let bytes = Bytes::from_static(string.as_bytes());
      Poll::Ready(Some(Ok(bytes)))
    }
  }
}

impl MessageBody for String {
  type Error = Infallible;

  fn size(&self) -> BodySize {
    BodySize::Sized(self.len() as u64)
  }

  fn poll_next(
    self: Pin<&mut Self>,
    _cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    if self.is_empty() {
      Poll::Ready(None)
    } else {
      let string = mem::take(self.get_mut());
      Poll::Ready(Some(Ok(Bytes::from(string))))
    }
  }
}
