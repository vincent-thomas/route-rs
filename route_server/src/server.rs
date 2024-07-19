use std::{
  io::{BufRead, BufReader, Read, Write},
  net::{SocketAddr, TcpStream},
  str::FromStr,
  sync::Arc,
};

use route::App;
use tokio::task;
// use http_body_util::Full;
// use hyper::{
//   body::{Bytes, Incoming},
//   header::HeaderValue,
//   service::Service,
//   Response,
// };
use std::net::TcpListener;

use route_http::{
  header::{HeaderMap, HeaderValue, CONTENT_LENGTH},
  request::{HttpRequest, Request},
};

use crate::{findable::FindableRoute, threadpool::ThreadPool};

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
    //let pool = ThreadPool::new(4);

    loop {
      let (stream, _) = listener.accept()?;

      let stream = Arc::new(stream);
      let stream = Arc::clone(&stream);
      let app = self.app.clone();

      task::spawn(async move {
        handle_connection(Arc::try_unwrap(stream).unwrap(), app).await;
      });

      // pool.execute(Box::new(move || {
      //   Box::pin(async {
      //     handle_connection(Arc::try_unwrap(stream).unwrap(), app).await;
      //   })
      // }));
    }
  }
}

async fn handle_connection(mut stream: TcpStream, app: Arc<App>) {
  let mut buf_reader = BufReader::new(&mut stream);
  let mut request_lines = Vec::new();

  // Read headers into `request_lines`
  loop {
    let mut line = String::new();
    if let Ok(0) = buf_reader.read_line(&mut line) {
      break; // Reached EOF
    }
    if line.trim().is_empty() {
      break; // Empty line indicates end of headers
    }
    request_lines.push(line.trim().to_string());
  }

  let http_meta = request_lines.swap_remove(0);

  let parts: Vec<&str> = http_meta.split_whitespace().collect();

  let method = route_http::Method::from_str(parts[0]).unwrap();
  let uri = route_http::uri::Uri::from_str(parts[1]).unwrap();

  let iter = request_lines.iter().map(|line| {
    let mut parts: Vec<&str> = line.split(": ").collect();
    let key = parts.remove(0);
    (key.parse().unwrap(), parts[0].to_string().parse().unwrap())
  });

  let headers: HeaderMap<HeaderValue> = HeaderMap::from_iter(iter);

  let body_length = headers.get(CONTENT_LENGTH);

  let body_bytes = if let Some(length) = body_length {
    let length = length.to_str().unwrap().parse().unwrap();
    let mut body = vec![0u8; length];
    buf_reader.read_exact(&mut body).unwrap();
    body
  } else {
    Vec::new()
  };

  let mut req_builder = Request::builder().uri(uri).method(method);

  req_builder.headers_mut().unwrap().extend(headers.into_iter());

  let req: HttpRequest = req_builder.body(body_bytes.into()).unwrap();

  println!("Input: {:#?}", req);

  let service = app.find_route(req.uri().path());

  stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();

  //let mut output = service.call_service(req).await;

  // if cfg!(debug_assertions) {
  //   output.headers_mut().insert("Server", HeaderValue::from_static("Route-RS"));
  // }
  // println!("Output: {:#?}", output);
}
