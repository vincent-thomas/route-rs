use std::{error::Error, io::Write};

use route_http::{
  body::BoxBody,
  request::Request,
  response::{HttpResponse, Response},
};

//#[async_trait::async_trait]
// pub trait RawHttpService: Sync + Send {
//   async fn call_rawservice(
//     &self,
//     req: Request,
//     stream: &mut dyn SendWrite,
//   ) -> Result<(), Box<dyn Error>>;
// }

#[async_trait::async_trait]
pub trait HttpService: Sync + Send {
  async fn call_service(&self, req: Request) -> HttpResponse<Box<[u8]>>;
}

#[async_trait::async_trait]
pub trait Service: Sync + Send {
  async fn call_service(&self, req: Request) -> Response<BoxBody>;
}

// #[async_trait::async_trait]
// impl<R: HttpService> RawHttpService for R {
//   async fn call_rawservice(
//     &self,
//     req: Request,
//     stream: &mut dyn SendWrite,
//   ) -> Result<(), Box<dyn Error>> {
//     let res = self.call_service(req).await;
//
//     let raw_res: String = HttpResponseExt(res).into();
//     stream.write_all(raw_res.as_bytes())?;
//
//     Ok(())
//   }
// }
