#[macro_export]
macro_rules! register_method {
  ($method:ident, $fn:ident) => {
    pub fn $fn<H, Args>(mut self, handler: H) -> Self
    where
      H: route_core::Handler<Args>,
      H::Output: route_core::Respondable,
      Args: route_core::FromRequest + 'static,
    {
      let endpoint = Endpoint::new(handler);
      self.method(route_http::Method::$method, endpoint);
      self
    }
  };
}
