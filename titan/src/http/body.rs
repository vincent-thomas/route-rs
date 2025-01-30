use std::{
  convert::Infallible,
  pin::{pin, Pin},
  task::Poll,
};

use bytes::Bytes;
use futures_core::Stream;
use http_body::{Frame, SizeHint};

/// Represents the body of an HTTP response.
///
/// The `Body` enum can either represent a fully-loaded body in memory or a stream of data.
/// It is used as the response body when implementing the `Respondable` trait, allowing
/// handlers to return either a complete body or a body that streams data incrementally.
///
/// # Variants
///
/// - `Full(Box<[u8]>)`:
///   This variant holds the full body of the response in memory as a boxed byte slice. This is typically used
///   when the response body is small enough to be loaded entirely into memory at once.
///   
/// - `Stream(Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>)`:
///   This variant represents a streaming body, where the body is returned incrementally in chunks. This is useful
///   when dealing with large responses (e.g., files or large datasets) that should be sent in multiple parts.
///   The stream yields `Vec<u8>` chunks, allowing the receiver to process the data incrementally as it arrives.
///
/// This enum is not usually used of the library user and is quite low-level.
pub enum Body {
  Full(Box<[u8]>),
  Stream(Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>),
}

impl http_body::Body for Body {
  type Data = Bytes;
  type Error = Infallible;

  fn size_hint(&self) -> http_body::SizeHint {
    match self {
      Body::Full(value) => SizeHint::with_exact(value.len() as u64),
      Body::Stream(body) => {
        let (lower, higher) = body.size_hint();
        let mut size_hint = SizeHint::default();
        size_hint.set_lower(lower as u64);
        if let Some(higher) = higher {
          size_hint.set_upper(higher as u64);
        }
        size_hint
      }
    }
  }

  fn poll_frame(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>>
  {
    match self.get_mut() {
      Body::Full(body) => {
        Poll::Ready(Some(Ok(Frame::data(Bytes::from(body.clone())))))
      }
      Body::Stream(body) => {
        let value = pin!(body);
        let value = match value.poll_next(cx) {
          Poll::Pending => return Poll::Pending,
          Poll::Ready(value) => match value {
            None => return Poll::Ready(None),
            Some(value) => value,
          },
        };

        let frame = Frame::data(Bytes::from(value));
        Poll::Ready(Some(Ok(frame)))
      }
    }
  }

  fn is_end_stream(&self) -> bool {
    // TODO
    true
  }
}

impl From<String> for Body {
  fn from(value: String) -> Self {
    Self::Full(value.as_bytes().into())
  }
}

impl From<()> for Body {
  fn from(_: ()) -> Self {
    Self::Full([].into())
  }
}

impl<'a> From<&'a str> for Body {
  fn from(value: &'a str) -> Self {
    Self::Full(value.as_bytes().into())
  }
}

impl From<Box<[u8]>> for Body {
  fn from(value: Box<[u8]>) -> Self {
    Self::Full(value)
  }
}

impl From<Vec<u8>> for Body {
  fn from(value: Vec<u8>) -> Self {
    Self::Full(value.into())
  }
}

impl From<&'_ [u8]> for Body {
  fn from(value: &'_ [u8]) -> Self {
    Self::Full(value.into())
  }
}

//impl From<StatusCode> for Body {
//  fn from(value: StatusCode) -> Self {
//    match value {
//      StatusCode::OK => "Ok",
//      StatusCode::BAD_REQUEST => "Bad Request",
//      _ => panic!("no"),
//    }
//    .into()
//  }
//}

//impl From<Body> for String {
//  fn from(value: Body) -> Self {
//    match value {
//      Body::Full(bytes) => unsafe {
//        String::from_utf8_unchecked(bytes.to_vec())
//      },
//    }
//  }
//}

macro_rules! impl_tostring {
  ($( $type:ident )*) => {
    $(impl From<$type> for Body {
          fn from(value: $type) -> Self {
            let body_str = value.to_string();
            Self::Full(body_str.as_bytes().into())
          }
    })*
  };
}

impl_tostring! { usize i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 }
