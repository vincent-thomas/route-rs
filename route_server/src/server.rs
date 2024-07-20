use std::{
  io::{BufReader, Read, Write},
  net::{SocketAddr, TcpStream},
  sync::Arc,
};

use crate::utils::{date_header_format, read_request};
use route::App;
use std::net::TcpListener;
use tokio::task;

use route_http::{
  header::{HeaderValue, CONTENT_LENGTH},
  request::HttpRequestExt,
  response::HttpResponseExt,
};

pub struct Server {
  socket: SocketAddr,
  #[allow(dead_code)]
  app: Arc<App>,
}

impl Server {
  pub(crate) fn new(socket: SocketAddr, app: App) -> Self {
    Server { socket, app: Arc::new(app) }
  }

  pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(self.socket)?;
    loop {
      let (stream, _) = listener.accept()?;
      let moved_app = self.app.clone();
      task::spawn(async move {
        handle_connection(stream, moved_app).await;
      });
    }
  }
}

async fn handle_connection(mut stream: TcpStream, app: Arc<App>) {
  let mut buf_reader = BufReader::new(&mut stream);
  let http_req = read_request(&mut buf_reader);

  let str_request = http_req.join("\n");
  let mut req = HttpRequestExt::from(str_request).0;

  dbg!(&req);

  let body_length = req.headers().get(CONTENT_LENGTH);

  let body_bytes = if let Some(length) = body_length {
    let length = length.to_str().unwrap().parse().unwrap();
    let mut body = vec![0u8; length];
    buf_reader.read_exact(&mut body).unwrap();
    body
  } else {
    Vec::new()
  };

  let content_length_header_value =
    body_length.cloned().unwrap_or(HeaderValue::from_static("0"));

  *req.body_mut() = body_bytes.into();

  let service = app.route(req.uri().path());

  let mut service_output = service.call_service(req).await;

  let service_headers = service_output.headers_mut();

  service_headers.insert("Content-Length", content_length_header_value);

  service_headers
    .insert("Date", HeaderValue::from_str(&date_header_format()).unwrap());
  service_headers.insert("Connection", HeaderValue::from_static("close"));

  if cfg!(debug_assertions) {
    service_headers.insert("Server", HeaderValue::from_static("Route-RS"));
  }

  let response: String = HttpResponseExt(service_output).into();
  stream.write_all(response.as_bytes()).unwrap();
}
