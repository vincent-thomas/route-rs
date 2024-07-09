#![allow(non_snake_case)]
use serde::Serialize;
mod respondable;
pub use respondable::*;

use crate::{http_header_to_httpresponse, new_httpresponse, status::StatusCode};

pub struct HttpResponse {
  pub status: StatusCode,
  pub headers: http::HeaderMap,
  pub body: bytes::Bytes,
}

impl HttpResponse {
  pub fn body_bytes(mut self, body: impl Into<bytes::Bytes>) -> Self {
    self.body = body.into();
    self
  }

  pub fn body<B>(mut self, body: B) -> Self
  where
    B: Serialize,
  {
    self.body = bytes::Bytes::from(serde_json::to_string(&body).unwrap());
    self
  }

  http_header_to_httpresponse!(host, HOST);
  http_header_to_httpresponse!(content_type, CONTENT_TYPE);
  http_header_to_httpresponse!(content_length, CONTENT_LENGTH);
  http_header_to_httpresponse!(accept, ACCEPT);
  http_header_to_httpresponse!(accept_language, ACCEPT_LANGUAGE);
  http_header_to_httpresponse!(accept_encoding, ACCEPT_ENCODING);
  http_header_to_httpresponse!(accept_charset, ACCEPT_CHARSET);
  http_header_to_httpresponse!(accept_ranges, ACCEPT_RANGES);
  http_header_to_httpresponse!(authorization, AUTHORIZATION);
  http_header_to_httpresponse!(cache_control, CACHE_CONTROL);
  http_header_to_httpresponse!(connection, CONNECTION);
  http_header_to_httpresponse!(date, DATE);
  http_header_to_httpresponse!(expect, EXPECT);
  http_header_to_httpresponse!(from, FROM);
  http_header_to_httpresponse!(if_match, IF_MATCH);
  http_header_to_httpresponse!(if_modified_since, IF_MODIFIED_SINCE);
  http_header_to_httpresponse!(if_none_match, IF_NONE_MATCH);
  http_header_to_httpresponse!(if_range, IF_RANGE);
  http_header_to_httpresponse!(if_unmodified_since, IF_UNMODIFIED_SINCE);
  http_header_to_httpresponse!(max_forwards, MAX_FORWARDS);
  http_header_to_httpresponse!(pragma, PRAGMA);
  http_header_to_httpresponse!(proxy_authorization, PROXY_AUTHORIZATION);
  http_header_to_httpresponse!(range, RANGE);
  http_header_to_httpresponse!(referer, REFERER);
  http_header_to_httpresponse!(te, TE);
  http_header_to_httpresponse!(upgrade, UPGRADE);
  http_header_to_httpresponse!(user_agent, USER_AGENT);
  http_header_to_httpresponse!(via, VIA);
  http_header_to_httpresponse!(warning, WARNING);
  http_header_to_httpresponse!(cookie, COOKIE);
  http_header_to_httpresponse!(set_cookie, SET_COOKIE);
  http_header_to_httpresponse!(origin, ORIGIN);
  http_header_to_httpresponse!(access_control_allow_origin, ACCESS_CONTROL_ALLOW_ORIGIN);
  http_header_to_httpresponse!(access_control_allow_methods, ACCESS_CONTROL_ALLOW_METHODS);
  http_header_to_httpresponse!(access_control_allow_headers, ACCESS_CONTROL_ALLOW_HEADERS);
  http_header_to_httpresponse!(access_control_allow_credentials, ACCESS_CONTROL_ALLOW_CREDENTIALS);
  http_header_to_httpresponse!(access_control_max_age, ACCESS_CONTROL_MAX_AGE);
  http_header_to_httpresponse!(access_control_expose_headers, ACCESS_CONTROL_EXPOSE_HEADERS);
  http_header_to_httpresponse!(access_control_request_method, ACCESS_CONTROL_REQUEST_METHOD);
  http_header_to_httpresponse!(access_control_request_headers, ACCESS_CONTROL_REQUEST_HEADERS);
  http_header_to_httpresponse!(alt_svc, ALT_SVC);
  http_header_to_httpresponse!(content_disposition, CONTENT_DISPOSITION);
  http_header_to_httpresponse!(content_encoding, CONTENT_ENCODING);
  http_header_to_httpresponse!(content_language, CONTENT_LANGUAGE);
  http_header_to_httpresponse!(content_location, CONTENT_LOCATION);
  http_header_to_httpresponse!(content_range, CONTENT_RANGE);
  http_header_to_httpresponse!(etag, ETAG);
  http_header_to_httpresponse!(expires, EXPIRES);
  http_header_to_httpresponse!(last_modified, LAST_MODIFIED);
  http_header_to_httpresponse!(link, LINK);
  http_header_to_httpresponse!(location, LOCATION);
  http_header_to_httpresponse!(proxy_authenticate, PROXY_AUTHENTICATE);
  http_header_to_httpresponse!(refresh, REFRESH);
  http_header_to_httpresponse!(retry_after, RETRY_AFTER);
  http_header_to_httpresponse!(strict_transport_security, STRICT_TRANSPORT_SECURITY);
  http_header_to_httpresponse!(trailer, TRAILER);
  http_header_to_httpresponse!(transfer_encoding, TRANSFER_ENCODING);
  http_header_to_httpresponse!(vary, VARY);
  http_header_to_httpresponse!(www_authenticate, WWW_AUTHENTICATE);
  http_header_to_httpresponse!(dnt, DNT);
  http_header_to_httpresponse!(content_security_policy, CONTENT_SECURITY_POLICY);
  http_header_to_httpresponse!(x_content_type_options, X_CONTENT_TYPE_OPTIONS);
  http_header_to_httpresponse!(x_frame_options, X_FRAME_OPTIONS);
}

new_httpresponse!(Ok Created Accepted NoContent BadRequest Unauthorized Forbidden NotFound MethodNotAllowed Conflict InternalServerError NotImplemented ServiceUnavailable GatewayTimeout ImATeapot);
