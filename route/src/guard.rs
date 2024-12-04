use route_core::Respondable;
use route_http::body::Body;
use route_http::{request::Parts, response::Response, StatusCode};

pub enum GuardReason {
  Unauthorized,
  Forbidden,
  BadRequest,
  NotFound,
  InternalServerError,
  Custom(String),
}

impl Respondable for GuardReason {
  fn respond(self) -> Response<Body> {
    let status = match self {
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      GuardReason::Custom(_) => StatusCode::BAD_REQUEST,
    };

    Response::builder().status(status).body(Body).unwrap()
  }
}

pub enum GuardOutcome {
  WeJustPassinBy,
  Reason(GuardReason),
}

pub trait Guard: Sync + Send {
  fn check(&self, head: &Parts) -> GuardOutcome;
}
