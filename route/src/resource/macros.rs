#[macro_export]
macro_rules! register_method {
  ($method:ident, $fn:ident) => {
    pub fn $fn<H, Args>(mut self, handler: H) -> Self
    where
      H: route_core::Handler<Args> + Send + Sync,
      H::Output: Respondable,
      Args: FromRequest + 'static + Send + Sync,
      H::Future: Send,
    {
      let endpoint = Endpoint::new(handler);
      self.method(route_http::Method::$method, endpoint);
      self
    }
  };
}
