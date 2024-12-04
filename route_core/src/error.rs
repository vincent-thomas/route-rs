use route_http::{ response::Response, StatusCode};

// pub struct FromRequestError {
//   cause: Box<dyn ResponseError>,
// }

pub trait ResponseError {
  fn status_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }

  fn error_response(&self) -> Response<BoxBody>;
}

// impl FromRequestError {
//   pub fn new(status: StatusCode, message: Box<[u8]>) -> Self {
//     Self { status, message }
//   }
//
//   pub fn checked_method(&self) -> &StatusCode {
//     match self.status.is_server_error() || self.status.is_client_error() {
//       true => &self.status,
//       false => {
//         // TODO: Log error
//         &StatusCode::INTERNAL_SERVER_ERROR
//       }
//     }
//   }
// }
