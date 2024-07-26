use std::{
  error::Error,
  pin::Pin,
  task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::{ready, Stream};
use route_utils::BoxStdError;

use super::BoxBody;

pub enum BodySize {
  None,
  Sized(u64),
  Stream,
}

pub type BoxMessageBody = Box<dyn MessageBody<Error = BoxStdError>>;

pub trait MessageBody {
  type Error: Into<Box<dyn Error>>;

  fn size(&self) -> BodySize;

  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>>;

  fn try_into_bytes(self) -> Result<Bytes, Self>
  where
    Self: Sized,
  {
    Err(self)
  }
  fn boxed(self) -> BoxBody
  where
    Self: Sized + 'static,
  {
    BoxBody::new(self)
  }
}

pin_project_lite::pin_project! {
    pub struct BodyStream<S> {
        #[pin]
        stream: S
    }
}

impl<S, E> BodyStream<S>
where
  S: Stream<Item = Result<Bytes, E>>,
  E: Into<BoxStdError> + 'static,
{
  #[inline]
  pub fn new(stream: S) -> Self {
    BodyStream { stream }
  }
}

impl<S, E> MessageBody for BodyStream<S>
where
  S: Stream<Item = Result<Bytes, E>>,
  E: Into<BoxStdError> + 'static,
{
  type Error = E;

  fn size(&self) -> BodySize {
    BodySize::Stream
  }

  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    loop {
      let stream = self.as_mut().project().stream;

      let chunk = match ready!(stream.poll_next(cx)) {
        Some(Ok(ref bytes)) if bytes.is_empty() => continue,
        opt => opt,
      };

      return Poll::Ready(chunk);
    }
  }
}

//
// impl Stream for StreamedMessageBody {
//   type Item = Result<Bytes, BoxStdError>;
//
//   fn poll_next(
//     self: Pin<&mut Self>,
//     cx: &mut Context<'_>,
//   ) -> Poll<Option<Self::Item>> {
//     let self_mut = self.get_mut();
//
//     Pin::new(&mut self_mut.0).poll_next(cx)
//   }
// }
