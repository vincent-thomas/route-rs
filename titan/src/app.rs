use crate::{prelude::*, route::Route, web};
use serde_json::Value;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use titan_core::{Respondable, Service};
use titan_http::{
  body::Body,
  request::Request,
  response::{Response, ResponseBuilder},
  StatusCode,
};
use titan_router::Router;
use titan_utils::BoxedSendFuture;

#[derive(Clone)]
pub struct App {
  inner: Arc<AppInner>,
}

async fn default_fallback() -> impl Respondable {
  (StatusCode::NOT_FOUND, "404 Not Found")
}

impl Default for App {
  fn default() -> Self {
    Self {
      inner: Arc::new(AppInner {
        router: Router::default(),
        fallback: BoxCloneService::new(Route::new(default_fallback)),
      }),
    }
  }
}

struct AppInner {
  router: Router<BoxCloneService<Request, Response, Response>>,
  fallback: BoxCloneService<Request, Response, Response>,
}

macro_rules! tap_inner {
    ( $self_:ident, mut $inner:ident => { $($stmt:stmt)* } ) => {
        #[allow(redundant_semicolons)]
        {
            let mut $inner = $self_.into_inner();
            $($stmt)*
            App {
                inner: Arc::new($inner),
            }
        }
    };
}

impl App {
  fn into_inner(self) -> AppInner {
    match Arc::try_unwrap(self.inner) {
      Ok(inner) => inner,
      Err(arc) => {
        AppInner { router: arc.router.clone(), fallback: arc.fallback.clone() }
      }
    }
  }
  pub fn fallback<H>(self, handler: H) -> Self
  where
    H: titan_core::Handler<()> + Sync + Clone,
    H::Future: std::future::Future<Output = H::Output> + Send,
    H::Output: titan_core::Respondable,
  {
    tap_inner!(self, mut this => {
        this.fallback = BoxCloneService::new(Route::new(handler));
    })
  }
  pub fn at<S>(self, path: &str, endpoint: S) -> Self
  where
    S: Service<
        Request,
        Response = Response,
        Error = Response,
        Future = BoxedSendFuture<Result<Response, Response>>,
      >
      + 'static
      + Clone
      + Sync
      + Send,
  {
    tap_inner!(self, mut this => {
      this.router.at(path, BoxCloneService::new(endpoint));
    })
  }
}

impl Service<Request> for App {
  type Response = Response<Body>;
  type Error = Response<Body>;
  type Future = BoxedSendFuture<Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    _cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    std::task::Poll::Ready(Ok(()))
  }

  fn call(&mut self, mut req: Request) -> Self::Future {
    let uri = req.uri().clone();
    match self.inner.router.lookup(uri.path()) {
      Some(endpoint) => {
        let params: HashMap<String, Value> =
          HashMap::from_iter(endpoint.params.iter().map(|(key, value)| {
            (key.to_string(), Value::from(value.to_string()))
          }));
        let mut extensions = titan_http::Extensions::new();
        extensions.insert(params);

        *req.extensions_mut() = extensions;

        let mut service = endpoint.value.clone();
        Box::pin(AppFuture { fut: service.call(req) })
      }
      None => {
        let mut fallback = self.inner.fallback.clone();
        Box::pin(AppFuture { fut: fallback.call(req) })
      }
    }
  }
}

pin_project_lite::pin_project! {
  struct AppFuture<F> {
      #[pin]
      fut: F
  }
}

impl<F> Future for AppFuture<F>
where
  F: Future,
{
  type Output = F::Output;

  fn poll(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let this = self.project();

    this.fut.poll(cx)
  }
}
