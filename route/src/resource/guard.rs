use route_http::{
  request::Head,
  response::Response,
  StatusCode,
};

use crate::{respond::{RespondableV2, Respondable}, body::BoxBody};

pub enum GuardReason {
  Unauthorized,
  Forbidden,
  BadRequest,
  NotFound,
  InternalServerError,
  Custom(String),
}

impl RespondableV2 for GuardReason {
  type Body = ();
  fn respond(self) -> Response {
    let status = match self {
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      GuardReason::Custom(_) => StatusCode::BAD_REQUEST,
    };

    Response::builder().status(status).body(BoxBody::Empty).unwrap()
  }
}

impl Respondable for GuardReason {
  fn respond(self) -> Response<Box<[u8]>> {
    let status = match self {
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      GuardReason::Custom(_) => StatusCode::BAD_REQUEST,
    };

    Response::builder().status(status).body([].into()).unwrap()
  }
}

pub enum GuardOutcome {
  WeJustPassinBy,
  Reason(GuardReason),
}

pub trait Guard: Sync + Send {
  fn check(&self, head: &Head) -> GuardOutcome;
}
