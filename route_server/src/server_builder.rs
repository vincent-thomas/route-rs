use std::{net::SocketAddr, str::FromStr};

use crate::{findable::FindableRoute, server::Server};

pub struct ServerBuilder {
  socket: SocketAddr,
}

pub struct NoApp;

impl ServerBuilder {
  pub fn bind(address: &'static str, port: u16) -> Self {
    let addr = format!("{}:{}", address, port);
    Self { socket: SocketAddr::from_str(&addr).unwrap() }
  }
}

impl ServerBuilder {
  pub fn app<A>(self, app: A) -> Server
  where
    A: FindableRoute + Send + Sync + 'static,
  {
    Server::new(self.socket, Box::new(app))
  }
}
