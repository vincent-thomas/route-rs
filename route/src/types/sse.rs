use futures_util::{Stream, StreamExt};
use route_core::Respondable;
use route_http::{
  body::Body,
  header::{HeaderMap, HeaderName, HeaderValue},
  response::Response,
};

#[derive(Clone)]
pub struct Sse<T>(pub T);

impl<T> Sse<T> {}

impl<T> Respondable for Sse<T>
where
  T: Stream<Item = Event> + Send + 'static,
{
  fn respond(self) -> route_http::response::Response<Body> {
    let stream = self.0.map(|item| {
      let t: String = item.into();
      t.as_bytes().to_vec()
    });
    let mut response = Response::new(Body::Stream(Box::pin(stream)));
    *response.headers_mut() = HeaderMap::from_iter([
      (
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/event-stream"),
      ),
      (
        HeaderName::from_static("cache-control"),
        HeaderValue::from_static("no-cache"),
      ),
      (
        HeaderName::from_static("connection"),
        HeaderValue::from_static("keep-alive"),
      ),
    ]);

    response
  }
}
#[derive(Clone)]
pub struct Event {
  data: Option<String>,
  event: Option<String>,
  id: Option<String>,
}

impl Event {
  pub fn new(data: String) -> Event {
    Event { data: Some(data), event: None, id: None }
  }
}

impl Into<String> for Event {
  fn into(self) -> String {
    let mut text = String::new();

    if let Some(data) = self.data {
      text.push_str(&format!("data: {data}\n"));
    };

    if let Some(id) = self.id {
      text.push_str(&format!("id: {id}\n"));
    };

    if let Some(event) = self.event {
      text.push_str(&format!("event: {event}\n"));
    };

    text.push('\n');

    text
  }
}
