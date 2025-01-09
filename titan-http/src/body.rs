use std::pin::Pin;

use futures_core::Stream;

pub enum Body {
  Full(Box<[u8]>),
  Stream(Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>),
}

impl Body {
  //pub fn len(&self) -> usize {
  //  match self {
  //    Self::Full(value) => value.len(),
  //  }
  //}

  //pub fn is_empty(&self) -> bool {
  //  match self {
  //    Self::Full(value) => value.is_empty(),
  //  }
  //}
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
    Self::Full(value.clone().into())
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
