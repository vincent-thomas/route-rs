use crate::IncomingStream;
use std::future::{poll_fn, IntoFuture};
use tokio::{io, net::TcpListener};
use tower_service::Service;

pub struct Serve<S> {
  pub(crate) listener: TcpListener,
  pub(crate) service: S,
}

impl<S> IntoFuture for Serve<S>
where
  S: for<'a> Service<IncomingStream<'a>> + 'static + Send + Clone,
  for<'a> <S as Service<IncomingStream<'a>>>::Future: Send,
{
  type Output = io::Result<()>;
  type IntoFuture = private::ServeFuture;

  fn into_future(self) -> Self::IntoFuture {
    private::ServeFuture(Box::pin(async move {
      let Self { mut service, listener } = self;
      loop {
        let mut tcp_stream = match utils::tcp_accept(&listener).await {
          Some(conn) => conn,
          None => continue,
        };

        if poll_fn(|cx| service.poll_ready(cx)).await.is_err() {
          eprintln!("Skipping running because poll_ready failed");
          continue;
        }

        let mut task_service = service.clone();
        tokio::spawn(async move {
          // For now...
          let incoming = IncomingStream(&mut tcp_stream);
          let _ = task_service.call(incoming).await;
        });
      }
    }))
  }
}

mod private {
  use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
  };

  pub struct ServeFuture(
    pub(super) Pin<Box<dyn Future<Output = io::Result<()>> + 'static>>,
  );

  impl Future for ServeFuture {
    type Output = io::Result<()>;

    #[inline]
    fn poll(
      mut self: Pin<&mut Self>,
      cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
      self.0.as_mut().poll(cx)
    }
  }

  impl std::fmt::Debug for ServeFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("ServeFuture").finish_non_exhaustive()
    }
  }
}

mod utils {
  use tokio::io;
  use tokio::net::{TcpListener, TcpStream};

  pub fn is_connection_error(e: &io::Error) -> bool {
    matches!(
      e.kind(),
      io::ErrorKind::ConnectionRefused
        | io::ErrorKind::ConnectionAborted
        | io::ErrorKind::ConnectionReset
    )
  }

  pub async fn tcp_accept(listener: &TcpListener) -> Option<TcpStream> {
    match listener.accept().await {
      Ok(conn) => Some(conn.0),
      Err(e) => {
        if !is_connection_error(&e) {
          eprintln!("Accept error: {e}");
        }
        None
      }
    }
  }
}

//response.headers_mut().extend([(
//  HeaderName::from_static("date"),
//  HeaderValue::from_str(&date_header_format()).unwrap(),
//)]);
//
//let (parts, body) = HttpResponseExt(response).parse_parts();
//
//tcp_stream.write_all(parts.as_bytes()).await.unwrap();
//tcp_stream.write_all(b"\r\n").await.unwrap();
//
//match body {
//  Body::Full(body) => {
//    tcp_stream.write_all(&body).await.unwrap();
//  }
//  Body::Stream(stream) => {
//    futures_util::pin_mut!(stream);
//
//    while let Some(chunk) = stream.next().await {
//      tcp_stream.write_all(&chunk).await.unwrap();
//      tcp_stream.flush().await.unwrap();
//    }
//    tcp_stream.shutdown().await.unwrap();
//  }
//}
//
//let mut buf_reader = BufReader::new(&mut tcp_stream);
//let http_headers =
//  utils::read_request(&mut buf_reader).await.join("\n");
//
//let req_empty_body = HttpRequestExt::from(http_headers).0;
//let body_length = req_empty_body
//  .headers()
//  .get(CONTENT_LENGTH)
//  .unwrap_or(&HeaderValue::from(0))
//  .to_str()
//  .unwrap()
//  .parse()
//  .unwrap();
//
//let req =
//  utils::fill_req_body(req_empty_body, body_length, buf_reader).await;
//let nice_service = service.clone();
//let mut nice_service = std::mem::replace(&mut service, nice_service);
//tokio::spawn(async move {
//  let mut response = match nice_service.call(req).await {
//    Ok(result) => result.respond(),
//    Err(result) => result.respond(),
//  };
//  response.headers_mut().extend([(
//    HeaderName::from_static("date"),
//    HeaderValue::from_str(&date_header_format()).unwrap(),
//  )]);
//
//  let (parts, body) = HttpResponseExt(response).parse_parts();
//
//  tcp_stream.write_all(parts.as_bytes()).await.unwrap();
//  tcp_stream.write_all(b"\r\n").await.unwrap();
//
//  match body {
//    Body::Full(body) => {
//      tcp_stream.write_all(&body).await.unwrap();
//    }
//    Body::Stream(stream) => {
//      futures_util::pin_mut!(stream);
//
//      while let Some(chunk) = stream.next().await {
//        tcp_stream.write_all(&chunk).await.unwrap();
//        tcp_stream.flush().await.unwrap();
//      }
//      tcp_stream.shutdown().await.unwrap();
//    }
//  }
//});
