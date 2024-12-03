use std::{
  error::Error,
  io::{BufReader, Read, Write as _},
  net::{SocketAddr, TcpListener, TcpStream},
  sync::Arc,
};

use route_core::service::Service;

use crate::utils::read_request;
use route::App;
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

  pub async fn run(self) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(self.socket)?;
    loop {
      let (stream, _) = listener.accept()?;
      let moved_app = self.app.clone();
      task::spawn(async move {
        /* if let Err(err) =  */
        handle_connection(stream, moved_app).await; /*  {
                                                      eprintln!("Error: {:?}", err);
                                                    } */
      });
    }
  }
}

async fn handle_connection(
  mut stream: TcpStream,
  app: Arc<App>,
) -> Result<(), Box<dyn Error>> {
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

  //let mut app_mut = app.lock().await;
  let response = app.call_service(req).await;

  let response_ext: String = HttpResponseExt(response).into();

  stream.write_all(response_ext.as_bytes()).unwrap();
  Ok(())

  //let service = app.route(req.uri().path());

  //let output = service.call_service(req).await;
  //
  //let (parts, body) = output.into_parts();
  //let pinned = pin!(body);
  //
  //let output = pinned.await;
  //
  //
  // let service_headers = service_output.headers_mut();
  //
  // service_headers.insert("Content-Length", content_length_header_value);
  //
  // service_headers
  //   .insert("Date", HeaderValue::from_str(&date_header_format()).unwrap());
  // service_headers.insert("Connection", HeaderValue::from_static("close"));
  //
  // if cfg!(debug_assertions) {
  //   service_headers.insert("Server", HeaderValue::from_static("Route-RS"));
  // }
  //
  // let response: String = HttpResponseExt(service_output).into();
  // stream.write_all(response.as_bytes()).unwrap();
}
