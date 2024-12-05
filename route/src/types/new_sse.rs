use futures_core::Stream;
use route_utils::BoxStdError;
use tokio::time::Interval;

pin_project_lite::pin_project! {
    pub struct NewSse<S> {
      keep_alive: Option<Interval>,
      #[pin]
      stream: S,
    }
}

pub enum Event {
  Data(Data),
  Comment(Bytes),
}

pub struct Data {
  id: Option<&'static str>,
  data: Bytes,
}

use bytes::{BufMut as _, Bytes, BytesMut};
impl Event {
  fn line_split_with_prefix(
    buf: &mut BytesMut,
    prefix: &'static str,
    data: Bytes,
  ) {
    // initial buffer size guess is len(data) + 10 lines of prefix + EOLs + EOF
    buf.reserve(data.len() + (10 * (prefix.len() + 1)) + 1);

    // append prefix + space + line to buffer
    for line in data.split(|&v| v == b'\n') {
      buf.put_slice(prefix.as_bytes());
      buf.extend_from_slice(line);
      buf.put_u8(b'\n');
    }
  }
  fn into_bytes(self) -> Bytes {
    let mut buf = BytesMut::new();

    match self {
      Self::Data(data) => {
        if let Some(id) = data.id {
          buf.put_slice(b"id: ");
          buf.put_slice(id.as_bytes());
          buf.put_slice(b"\n");
        }

        for line in data.data.split(|&b| b == b'\n') {
          buf.put_slice(b"data: ");
          buf.put_slice(line);
          buf.put_slice(b"\n");
        }

        Self::line_split_with_prefix(&mut buf, "data: ", data.data);
      }
      Self::Comment(text) => Self::line_split_with_prefix(&mut buf, ": ", text),
    }
    buf.put_u8(b'\n');
    buf.freeze()
  }
}

use std::task::Poll;

impl<S, E> MessageBody for NewSse<S>
where
  S: Stream<Item = Result<Event, E>>,
  E: Into<BoxStdError>,
{
  type Error = E;

  fn size(&self) -> BodySize {
    BodySize::Stream
  }

  fn poll_next(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Result<bytes::Bytes, Self::Error>>> {
    let this = self.project();

    if let Poll::Ready(msg) = this.stream.poll_next(cx) {
      return match msg {
        Some(Ok(data)) => Poll::Ready(Some(Ok(data.into_bytes()))),
        Some(Err(e)) => Poll::Ready(Some(Err(e))),
        None => Poll::Ready(None),
      };
    }

    Poll::Pending
  }
}
