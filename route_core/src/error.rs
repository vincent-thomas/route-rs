use route_http::StatusCode;

pub struct Error {
  status: StatusCode,
  pub message: Box<[u8]>,
}

impl Error {
  pub fn new(status: StatusCode, message: Box<[u8]>) -> Self {
    Self { status, message }
  }

  pub fn checked_method(&self) -> &StatusCode {
    match self.status.is_server_error() || self.status.is_client_error() {
      true => &self.status,
      false => {
        // TODO: Log error
        &StatusCode::INTERNAL_SERVER_ERROR
      }
    }
  }
}
