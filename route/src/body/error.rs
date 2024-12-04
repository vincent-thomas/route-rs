use std::{
  pin::Pin,
  task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::ready;
use pin_project_lite::pin_project;
use route_utils::BoxStdError;

use super::{BodySize, MessageBody};

pub struct BodyInfallible;

impl Into<BoxStdError> for BodyInfallible {
  fn into(self) -> BoxStdError {
    unreachable!()
  }
}

pin_project! {
    pub(crate) struct MessageBodyMapErr<B, F> {
        #[pin]
        body: B,
        mapper: Option<F>,
    }
}

impl<B, F, E> MessageBodyMapErr<B, F>
where
  B: MessageBody,
  F: FnOnce(B::Error) -> E,
{
  pub(crate) fn new(body: B, mapper: F) -> Self {
    Self { body, mapper: Some(mapper) }
  }
}

impl<B, F, E> MessageBody for MessageBodyMapErr<B, F>
where
  B: MessageBody,
  F: FnOnce(B::Error) -> E,
  E: Into<BoxStdError>,
{
  type Error = E;

  #[inline]
  fn size(&self) -> BodySize {
    self.body.size()
  }

  fn poll_next(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Bytes, Self::Error>>> {
    let this = self.as_mut().project();

    match ready!(this.body.poll_next(cx)) {
      Some(Err(err)) => {
        let f = self.as_mut().project().mapper.take().unwrap();
        let mapped_err = (f)(err);
        Poll::Ready(Some(Err(mapped_err)))
      }
      Some(Ok(val)) => Poll::Ready(Some(Ok(val))),
      None => Poll::Ready(None),
    }
  }
}
