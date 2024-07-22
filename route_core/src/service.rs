use std::{error::Error, io::Write};

use route_http::{
  request::HttpRequest,
  response::{HttpResponse, HttpResponseExt},
};

#[async_trait::async_trait]
pub trait RawHttpService: Sync + Send {
  async fn call_rawservice(
    &self,
    req: HttpRequest,
    stream: &mut dyn SendWrite,
  ) -> Result<(), Box<dyn Error>>;
}

#[async_trait::async_trait]
pub trait HttpService: Sync + Send {
  async fn call_service(&self, req: HttpRequest) -> HttpResponse;
}

pub trait SendWrite: Write + Send {}

impl<T: Write + Send> SendWrite for T {}

#[async_trait::async_trait]
impl<R: HttpService> RawHttpService for R {
  async fn call_rawservice(
    &self,
    req: HttpRequest,
    stream: &mut dyn SendWrite,
  ) -> Result<(), Box<dyn Error>> {
    let res = self.call_service(req).await;

    let raw_res: String = HttpResponseExt(res).into();
    stream.write_all(raw_res.as_bytes())?;

    Ok(())
  }
}
