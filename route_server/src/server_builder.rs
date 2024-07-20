use std::{net::SocketAddr, str::FromStr};

use route::App;

use crate::server::Server;

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
  pub fn app(self, app: App) -> Server {
    Server::new(self.socket, app)
  }
}
