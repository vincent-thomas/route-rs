use tokio::sync::broadcast::Sender;
pub struct LongPollingResource<T> {
  retry_after: i32,
  sender: Sender<T>,
}

impl<T> LongPollingResource<T> {
  pub fn new() -> (Self, Sender<T>) {
    let default = Self::default();
    let sender = default.sender.clone();
    (default, sender)
  }
}

impl<T> From<Sender<T>> for LongPollingResource<T> {
  fn from(sender: Sender<T>) -> Self {
    Self { retry_after: 50_000, sender }
  }
}

impl<T> Default for LongPollingResource<T> {
  fn default() -> LongPollingResource<T> {
    let sender = Sender::new(1);
    Self { retry_after: 50_000, sender }
  }
}

macro_rules! impl_serialize_type {
  ($package:ident, $struct_name:ident, $content_type:expr) => {
    #[async_trait]
    impl<T> RawHttpService for LongPollingResource<$struct_name<T>>
    where
      T: Clone + Send + Serialize,
    {
      async fn call_rawservice(
        &self,
        _req: Request,
        stream: &mut dyn SendWrite,
      ) -> Result<(), Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", $content_type.parse().unwrap());
        headers.insert("Cache-Control", "no-cache".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers
          .insert("Retry-After", self.retry_after.to_string().parse().unwrap());

        let head = Head { status: StatusCode::OK, headers };
        let head_str: String = head.into();

        stream.write_all(head_str.as_bytes())?;
        stream.write_all(b"\n")?;

        let mut reciever = self.sender.subscribe();

        let msg_result = reciever.recv().await;
        match msg_result {
          Ok(msg) => {
            let msg_str = $package::to_string(&msg)?;
            stream.write_all(msg_str.as_bytes())?;
            Ok(())
          }
          Err(e) => {
            dbg!(&e);
            Err(Box::new(e))
          }
        }
      }
    }
  };
}
// impl_serialize_type!(serde_json, Json, "application/json");
// impl_serialize_type!(
//   serde_urlencoded,
//   UrlEncoded,
//   "application/xxx-url-encoded"
// );
