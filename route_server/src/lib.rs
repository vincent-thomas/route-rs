mod serve;

use serve::Serve;
use tokio::net::{TcpListener, TcpStream};
use tower_service::Service;

pub struct IncomingStream<'a>(pub &'a mut TcpStream);

pub fn serve<S>(listener: TcpListener, service: S) -> Serve<S>
where
  S: for<'a> Service<IncomingStream<'a>> + 'static + Send + Clone,
  for<'a> <S as Service<IncomingStream<'a>>>::Future: Send,
{
  Serve { listener, service }
}
