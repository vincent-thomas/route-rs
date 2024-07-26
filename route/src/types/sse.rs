use tokio::sync::broadcast::{self, Sender};

pub struct SSEResource<T> {
  retry_after: i32,
  sender: broadcast::Sender<T>,
}

impl<T> SSEResource<T> {
  pub fn new() -> (Self, Sender<T>)
  where
    T: Clone,
  {
    let default = SSEResource::default();
    let sender = default.sender.clone();
    (default, sender)
  }
}

impl<T> From<Sender<T>> for SSEResource<T> {
  fn from(sender: Sender<T>) -> Self {
    Self { retry_after: 50_000, sender }
  }
}

impl<T> Default for SSEResource<T>
where
  T: Clone,
{
  fn default() -> Self {
    Self { retry_after: 50_000, sender: Sender::new(1) }
  }
}

// macro_rules! impl_serialize_type {
//   ($package:ident, $struct_name:ident) => {
//     #[async_trait]
//     impl<T> RawHttpService for SSEResource<$struct_name<T>>
//     where
//       T: Serialize + Send + Clone,
//     {
//       async fn call_rawservice(
//         &self,
//         _req: HttpRequest,
//         stream: &mut dyn SendWrite,
//       ) -> Result<(), Box<dyn Error>> {
//         let mut headers = HeaderMap::new();
//         headers.insert("Content-Type", "text/event-stream".parse().unwrap());
//         headers.insert("Cache-Control", "no-cache".parse().unwrap());
//         headers.insert("Connection", "keep-alive".parse().unwrap());
//         headers
//           .insert("Retry-After", self.retry_after.to_string().parse().unwrap());
//
//         let head = Head { status: StatusCode::OK, headers };
//         let head_str: String = head.into();
//
//         stream.write_all(head_str.as_bytes())?;
//         stream.write_all(b"\n")?;
//
//         let mut reciever = self.sender.subscribe();
//
//         loop {
//           let msg_result = reciever.recv().await;
//
//           match msg_result {
//             Ok(msg) => {
//               let data_str = $package::to_string(&msg)?;
//               let test = format!("data: {}\n\n", data_str);
//               stream.write_all(test.as_bytes())?;
//             }
//             Err(e) => {
//               return Err(Box::new(e));
//             }
//           };
//         }
//       }
//     }
//   };
// }
// impl_serialize_type!(serde_json, Json);
// impl_serialize_type!(serde_urlencoded, UrlEncoded);
