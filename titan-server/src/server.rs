use std::future::{poll_fn, IntoFuture};

use futures_util::StreamExt as _;
use titan_core::{Respondable, Service};

use titan_http::{
  body::Body,
  header::{HeaderValue, CONTENT_LENGTH},
  HttpRequestExt, HttpResponseExt, Request,
};
use tokio::{
  io::{self, AsyncWriteExt as _, BufReader},
  net::{TcpListener, TcpStream},
};

use crate::utils::{self};

/// Starts a server that listens on the provided `TcpListener` and handles requests using the given `service`.
///
/// # Type Parameters
/// - `S`: The type of the service that processes incoming requests. It must implement the [`Service`] trait
///   for requests of type `Request<Box<[u8]>>`.
///
/// # Parameters
/// - `listener`: A [`tokio::net::TcpListener`] instance used to accept incoming TCP connections.
/// - `service`: An implementation of the [`Service`] trait to handle the requests.
///
/// # Constraints
/// - `S`: The service must:
///   - Implement the [`Service`] trait for `Request<Box<[u8]>>`.
///   - Be both [`Send`] and [`Clone`].
///   - Have a `'static` lifetime.
/// - The associated [`std::future::Future`] type of the service must be [`Send`].
/// - The associated `Response` and `Error` types of the service must implement [`Respondable`].
///
/// # Returns
/// A [`Serve`] instance that drives the server.
///
/// # Examples
/// ```
/// use titan_server::{serve};
/// use titan_core::{Respondable,Service};
/// use titan_http::Request;
/// use std::{future::Future, task::Poll, pin::Pin};
/// use tokio::net::TcpListener;
///
/// #[derive(Clone)]
/// struct MyService;
///
/// impl Service<Request> for MyService {
///     type Response = &'static str;
///     type Error = ();
///     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
///
///  fn poll_ready(
///   &mut self,
///   _cx: &mut std::task::Context<'_>,
///  ) -> Poll<Result<(), Self::Error>> {
///    Poll::Ready(Ok(()))
/// }
///
///     fn call(&mut self, req: Request) -> Self::Future {
///         // Process the request and return a future
///         Box::pin(async move {Ok("testing")})
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
///   let service = MyService;
///   
///   // Uncomment the last part in your app
///   serve(listener, service); // .await.unwrap();
/// }
///
/// ```
///
/// # Errors
/// Any errors from the underlying network layer or service will be propagated and should be handled appropriately.
///
/// # See Also
/// - [`TcpListener`]: For details on how to set up a TCP listener.
/// - [`Service`]: For implementing request handling logic.
/// - [`Respondable`]: For implementing custom response and error types.
pub fn serve<S>(listener: TcpListener, service: S) -> Serve<S>
where
  S: titan_core::Service<Request> + Send + Clone + 'static,
  S::Future: Send,
  S::Response: Respondable,
  S::Error: Respondable,
{
  Serve { listener, service }
}

pub struct Serve<S> {
  listener: TcpListener,
  service: S,
}

fn is_connection_error(e: &io::Error) -> bool {
  matches!(
    e.kind(),
    io::ErrorKind::ConnectionRefused
      | io::ErrorKind::ConnectionAborted
      | io::ErrorKind::ConnectionReset
  )
}

impl<S> Serve<S> {
  async fn tcp_accept(listener: &TcpListener) -> Option<TcpStream> {
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

impl<S> IntoFuture for Serve<S>
where
  S: Service<Request> + 'static + Send + Clone,
  S::Future: Send,
  S::Error: Respondable,
  S::Response: Respondable,
{
  type Output = io::Result<()>;
  type IntoFuture = private::ServeFuture;

  fn into_future(self) -> Self::IntoFuture {
    private::ServeFuture(Box::pin(async move {
      let Self { mut service, listener } = self;
      loop {
        let mut tcp_stream = match Self::tcp_accept(&listener).await {
          Some(conn) => conn,
          None => continue,
        };

        if poll_fn(|cx| service.poll_ready(cx)).await.is_err() {
          eprintln!("Skipping running because poll_ready failed");
          continue;
        }

        let mut buf_reader = BufReader::new(&mut tcp_stream);
        let http_headers =
          utils::read_request(&mut buf_reader).await.join("\n");

        let req_empty_body = HttpRequestExt::from(http_headers).0;
        let body_length = req_empty_body
          .headers()
          .get(CONTENT_LENGTH)
          .unwrap_or(&HeaderValue::from(0))
          .to_str()
          .unwrap()
          .parse()
          .unwrap();

        let req =
          utils::fill_req_body(req_empty_body, body_length, buf_reader).await;
        let nice_service = service.clone();
        let mut nice_service = std::mem::replace(&mut service, nice_service);
        tokio::spawn(async move {
          #[allow(unused_mut)]
          let mut response = match nice_service.call(req).await {
            Ok(result) => result.respond(),
            Err(result) => result.respond(),
          };

          #[cfg(feature = "date-header")]
          {
            use titan_http::header::HeaderName;
            response.headers_mut().extend([(
              HeaderName::from_static("date"),
              HeaderValue::from_str(&chrono::Utc::now()
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string()())
              .unwrap(),
            )]);
          }

          let (parts, body) = HttpResponseExt(response).parse_parts();

          tcp_stream.write_all(parts.as_bytes()).await.unwrap();
          tcp_stream.write_all(b"\r\n").await.unwrap();

          match body {
            Body::Full(body) => {
              tcp_stream.write_all(&body).await.unwrap();
            }
            Body::Stream(stream) => {
              futures_util::pin_mut!(stream);

              while let Some(chunk) = stream.next().await {
                tcp_stream.write_all(&chunk).await.unwrap();
                tcp_stream.flush().await.unwrap();
              }
              tcp_stream.shutdown().await.unwrap();
            }
          }
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
