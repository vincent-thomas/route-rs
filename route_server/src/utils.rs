use std::{
  io::{BufRead, BufReader, Read},
  net::TcpStream,
};

use chrono::Utc;
use route_http::request::Request;

pub(crate) fn read_request(
  reader: &mut BufReader<&mut TcpStream>,
) -> Vec<String> {
  let mut request_lines = Vec::new();
  loop {
    let mut line = String::new();
    if let Ok(0) = reader.read_line(&mut line) {
      // End of buffer if 0 bytes left.
      break;
    }
    if line.trim().is_empty() {
      break;
    }
    request_lines.push(line.trim().to_string());
  }
  request_lines
}

pub(crate) fn date_header_format() -> String {
  Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub(crate) fn fill_req_body(
  mut req: Request,
  body_length: usize,
  mut reader: BufReader<&mut TcpStream>,
) -> Request {
  if body_length == 0 {
    return req;
  };
  let mut body = vec![0u8; body_length];
  reader.read_exact(&mut body).unwrap();
  *req.body_mut() = body.into();
  req
}
