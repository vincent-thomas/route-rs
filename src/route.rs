use route_core::Endpoint;

pub struct Route {
  endpoint: Box<dyn Endpoint>,
}
