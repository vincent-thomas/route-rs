#[macro_export]
macro_rules! http_header_to_httpresponse {
  ($($name:ident, $key:ident)*) => {
    $(
      pub fn $name(mut self, value: &str) -> Self {
        self.headers.insert(http::header::$key, value.parse().unwrap());
        self
      }
    )*
  };
}

#[macro_export]
macro_rules! new_httpresponse {
  ($($name:ident)*) => {
    impl HttpResponse {
      $(
        pub fn $name() -> HttpResponse {
          HttpResponse { status: StatusCode::$name, body: bytes::Bytes::new(), headers: http::HeaderMap::new() }
        }
      )*
    }
  };
}

#[macro_export]
macro_rules! new_status_code {
  ($($name:ident)*) => {
    impl StatusCode {
      $(
        pub fn $name() -> StatusCode {
          StatusCode::$name
        }
      )*
    }
  };
}

#[macro_export]
macro_rules! status_code_to_int {
  ($($name:ident => $int:expr),*) => {
    impl crate::response::StatusCode {
      pub fn to_int(status: crate::response::StatusCode) -> u16 {
        match status {
          crate::response::StatusCode::Other(int) => int,
          $(
            crate::response::StatusCode::$name => $int,
          )*
        }
      }
    }
  };
}
