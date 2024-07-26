use std::pin::Pin;

use bytes::Bytes;

use super::{error::MessageBodyMapErr, BodySize, MessageBody};
use route_utils::BoxStdError;

pub enum BoxBody {
  Empty,
  Full(Bytes),
  Stream(Pin<Box<dyn MessageBody<Error = BoxStdError>>>),
}

impl BoxBody {
  pub fn new<B>(body: B) -> Self
  where
    B: MessageBody + 'static,
  {
    match body.size() {
      BodySize::None => BoxBody::Empty,
      _ => match body.try_into_bytes() {
        Ok(bytes) => Self::Full(bytes),
        Err(body) => {
          let body = MessageBodyMapErr::new(body, Into::into);
          Self::Stream(Box::pin(body))
        }
      },
    }
  }
}

impl MessageBody for BoxBody {
  type Error = BoxStdError;

  fn size(&self) -> BodySize {
    match *self {
      BoxBody::Empty => BodySize::Sized(0),
      BoxBody::Full(ref bytes) => bytes.size(),
      BoxBody::Stream(ref body) => body.size(),
    }
  }
  fn poll_next(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Result<Bytes, Self::Error>>> {
    match self.get_mut() {
      BoxBody::Empty => std::task::Poll::Ready(None),
      BoxBody::Full(bytes) => {
        Pin::new(bytes).poll_next(cx).map_err(|err| match err {})
      }

      BoxBody::Stream(body) => Pin::new(body).poll_next(cx),
    }
  }
}
