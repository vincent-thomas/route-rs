use route_core::service::HttpService;

pub trait FindableRoute<'a> {
  fn find_route(&'a self, path: &str) -> Box<dyn HttpService>;
}
