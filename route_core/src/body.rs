use std::{error::Error, io::Write, net::TcpStream, pin::Pin};

use futures::{Stream, StreamExt};
use route_http::response::Response;

async fn demo<T>(
  mut input: Response<T>,
  writer: &mut TcpStream,
) -> Result<(), Box<dyn Error>>
where
  T: Stream<Item = Box<[u8]>> + Unpin,
{
  let mut body = input.body_mut();
  let mut pinned_body = Pin::new(&mut body);

  while let Some(next_chunk) = pinned_body.next().await {
    writer.write_all(&next_chunk)?;
    writer.flush()?;
  }

  Ok(())
}
