use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};

use chrono::Utc;

pub(crate) fn read_request(
  reader: &mut BufReader<&mut TcpStream>,
) -> Vec<String> {
  let mut request_lines = Vec::new();
  loop {
    let mut line = String::new();
    if let Ok(0) = reader.read_line(&mut line) {
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
