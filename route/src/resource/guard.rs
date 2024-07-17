use route_core::Respondable;
use route_http::{request::Head, response::HttpResponse, StatusCode};

pub enum GuardReason {
  Unauthorized,
  Forbidden,
  BadRequest,
  NotFound,
  InternalServerError,
  Custom(String),
}

impl Respondable for GuardReason {
  fn respond(self) -> HttpResponse {
    let mut res = HttpResponse::new([].into());
    *res.status_mut() = match self {
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      GuardReason::Custom(_) => StatusCode::BAD_REQUEST,
    };
    res
  }
}

pub enum GuardOutcome {
  WeJustPassinBy,
  Reason(GuardReason),
}

pub trait Guard {
  fn check(&self, head: &Head) -> GuardOutcome;
}
