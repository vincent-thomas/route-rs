use route_core::Respondable;
use route_http::body::Body;
use route_http::{request::Parts, response::Response, StatusCode};

pub trait Guard: Sync + Send {
  fn check(&self, head: &Parts) -> GuardOutcome;
}

pub enum GuardReason {
  Unauthorized,
  Forbidden,
  BadRequest,
  NotFound,
  InternalServerError,
  Custom((StatusCode, String)),
}

impl From<GuardReason> for Response<Body> {
  fn from(value: GuardReason) -> Self {
    let body = match value {
      GuardReason::Custom((_, ref text)) => Body::from(text.clone()),
      GuardReason::Unauthorized => Body::from("Unauthorized".to_string()),
      GuardReason::NotFound => Body::from("Not Found".to_string()),
      GuardReason::Forbidden => Body::from("Forbidden".to_string()),
      GuardReason::BadRequest => Body::from("Bad Request".to_string()),
      GuardReason::InternalServerError => {
        Body::from("Internal Server Error".to_string())
      }
    };
    let status = match value {
      GuardReason::Custom((status, _)) => status,
      GuardReason::Unauthorized => StatusCode::UNAUTHORIZED,
      GuardReason::NotFound => StatusCode::NOT_FOUND,
      GuardReason::Forbidden => StatusCode::FORBIDDEN,
      GuardReason::BadRequest => StatusCode::BAD_REQUEST,
      GuardReason::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let mut response = Response::new(body);
    *response.status_mut() = status;
    response
  }
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

    let body = Body::from(());
    Response::builder().status(status).body(body).unwrap()
  }
}

pub enum GuardOutcome {
  WeJustPassinBy,
  Reason(GuardReason),
}
