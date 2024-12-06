use std::{
  error::Error,
  io::{BufReader, Write as _},
  net::{SocketAddr, TcpListener, TcpStream},
  str::FromStr,
};

use route_core::{Respondable, Service};

use route_http::{
  header::{HeaderName, HeaderValue, CONTENT_LENGTH},
  request::{HttpRequestExt, Request},
  response::HttpResponseExt,
};

use crate::utils::{self, date_header_format};

/// Route
pub struct Server {
  socket: SocketAddr,
}

impl Server {
  pub fn bind(address: &'static str, port: u16) -> Self {
    let addr = format!("{}:{}", address, port);
    Server { socket: SocketAddr::from_str(&addr).unwrap() }
  }

  pub async fn run<S>(self, mut service: S) -> Result<(), Box<dyn Error>>
  where
    S: route_core::Service<Request> + Send + 'static,
    S::Response: Respondable,
    S::Error: Respondable,
  {
    let listener = TcpListener::bind(self.socket)?;
    loop {
      let (stream, _) = listener.accept()?;
      Self::handle_connection(stream, &mut service).await;
    }
  }

  async fn handle_connection<S>(mut stream: TcpStream, service: &mut S)
  where
    S: Service<Request>,
    S::Response: Respondable,
    S::Error: Respondable,
  {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_headers = utils::read_request(&mut buf_reader).join("\n");

    let req_empty_body = HttpRequestExt::from(http_headers).0;

    let body_length = req_empty_body
      .headers()
      .get(CONTENT_LENGTH)
      .unwrap_or(&HeaderValue::from(0))
      .to_str()
      .unwrap()
      .parse()
      .unwrap();

    let req = utils::fill_req_body(req_empty_body, body_length, buf_reader);

    let mut response = match Service::call(service, req).await {
      Ok(value) => value.respond(),
      Err(err) => err.respond(),
    };

    response.headers_mut().extend([
      (
        HeaderName::from_static("date"),
        HeaderValue::from_str(&date_header_format()).unwrap(),
      ),
      (
        HeaderName::from_static("server"),
        HeaderValue::from_str("route-rs").unwrap(),
      ),
    ]);

    let response_ext: String = HttpResponseExt(response).into();
    stream.write_all(response_ext.as_bytes()).unwrap();
  }
}
