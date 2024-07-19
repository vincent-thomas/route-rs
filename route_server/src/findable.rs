use route_core::service::HttpService;

pub trait FindableRoute: Send + Sync {
  fn find_route(&self, path: &str) -> Box<dyn HttpService>;
}
