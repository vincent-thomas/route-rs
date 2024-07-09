#![allow(non_snake_case)]

use crate::{new_status_code, status_code_to_int};

pub enum StatusCode {
  Ok,
  Created,
  Accepted,
  NoContent,
  BadRequest,
  Unauthorized,
  Forbidden,
  NotFound,
  MethodNotAllowed,
  Conflict,
  InternalServerError,
  NotImplemented,
  ServiceUnavailable,
  GatewayTimeout,

  ImATeapot,
  Other(u16),
}

new_status_code!(Ok Created Accepted NoContent BadRequest Unauthorized Forbidden NotFound MethodNotAllowed Conflict InternalServerError NotImplemented ServiceUnavailable GatewayTimeout ImATeapot);

status_code_to_int!(
  Ok => 200,
  Created => 201,
  Accepted => 202,
  NoContent => 204,
  BadRequest => 400,
  Unauthorized => 401,
  Forbidden => 403,
  NotFound => 404,
  MethodNotAllowed => 405,
  Conflict => 409,
  InternalServerError => 500,
  NotImplemented => 501,
  ServiceUnavailable => 503,
  GatewayTimeout => 504,
  ImATeapot => 418
);
