use route_core::Service;
use route_http::request::Request;
use route_utils::{BoxedFuture, BoxedSendFuture};

pub(crate) type BoxedSendService<Res> = Box<
  dyn Service<
    Request,
    Response = Res,
    Error = Res,
    Future = BoxedSendFuture<Result<Res, Res>>,
  >,
>;

pub(crate) type BoxedService<Res> = Box<
  dyn Service<
    Request,
    Response = Res,
    Error = Res,
    Future = BoxedFuture<Result<Res, Res>>,
  >,
>;
