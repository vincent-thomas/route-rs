use futures_util::StreamExt;
use std::path::PathBuf;
use titan_core::Service;

use titan_http::body::Body;
use titan_http::header;
use titan_http::Request;

use std::fs;
use std::io::Write;
pub async fn build_static(app: crate::App, path: PathBuf) {
  let inner_app = app.into_inner();

  for route in inner_app.router.into_iter() {
    if route.0.num_params() != 0 {
      panic!("Error: Can't statically build a dynamic path: {}", route.0)
    }

    let request = Request::new(Box::new([]));

    let response = match route.1.clone().call(request).await {
      Ok(value) => value,
      Err(_) => {
        panic!("Handler error in path: {:?}", route.0)
      }
    };

    let (parts, body) = response.into_parts();

    let bodyv2 = match body {
      Body::Full(body) => body,
      Body::Stream(mut stream) => {
        let mut full_body_message = Vec::default();
        while let Some(value) = stream.next().await {
          full_body_message.extend(value)
        }

        full_body_message.into_boxed_slice()
      }
    };

    if let Some(value) = parts.headers.get(header::CONTENT_TYPE) {
      if value != header::HeaderValue::from_str("text/html").unwrap() {
        panic!("only works on html for now");
      }
    } else {
      panic!("wtf");
    }

    let text_string = String::from_utf8(bodyv2.to_vec()).unwrap();

    let mut path = path.clone();

    let test = match route.0.to_string().as_str() {
      "/" => "index".to_string(),
      _ => route.0.to_string().replacen("/", "", 1),
    };

    path.push(format!("{}.html", test));

    fs::create_dir_all(path.parent().unwrap()).unwrap();

    let mut nice =
      fs::OpenOptions::new().create(true).write(true).open(path).unwrap();

    nice.write_all(text_string.as_bytes()).unwrap();
  }
}
