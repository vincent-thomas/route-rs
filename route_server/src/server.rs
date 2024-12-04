use std::{
  error::Error,
  future::Future,
  io::{BufReader, Read, Write as _},
  net::{SocketAddr, TcpListener, TcpStream},
  pin::{self, Pin},
  str::FromStr,
  sync::Arc,
};

use route::Respondable;
use route_core::service::Service;

use crate::utils::read_request;
use tokio::task;

use route_http::{
  header::{HeaderValue, CONTENT_LENGTH},
  request::HttpRequestExt,
  response::HttpResponseExt,
};

pub struct Server {
  socket: SocketAddr,
}

impl Server {
  pub fn bind(address: &'static str, port: u16) -> Self {
    let addr = format!("{}:{}", address, port);
    Server { socket: SocketAddr::from_str(&addr).unwrap() }
  }

  pub async fn run<S>(self, s: Arc<S>) -> Result<(), Box<dyn Error>>
  where
    S: Service + Send + Sync + 'static,
    S::Response: Respondable,
    S::Future: Send,
  {
    let listener = TcpListener::bind(self.socket)?;
    loop {
      let (stream, _) = listener.accept()?;
      let thing = s.clone();
      task::spawn(Self::handle_connection(stream, thing));
    }
  }

  async fn handle_connection<S>(mut stream: TcpStream, app: Arc<S>)
  where
    S: Service + Send + 'static,
    S::Response: Respondable,
    S::Future: Send,
  {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_req = read_request(&mut buf_reader);

    let str_request = http_req.join("\n");
    let mut req = HttpRequestExt::from(str_request).0;

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

    let result = app.call_service(req).await;
    let response = result.respond();

    let response_ext: String = HttpResponseExt(response).into();

    stream.write_all(response_ext.as_bytes()).unwrap();
  }
}
