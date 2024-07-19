use std::{
  future::Future,
  pin::Pin,
  sync::{mpsc, Arc, Mutex},
  thread::{self, JoinHandle},
};

struct Worker {
  id: usize,
  handle: JoinHandle<()>,
}
impl Worker {
  pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let handle = thread::spawn(move || {
      let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name(format!("route-worker-{}", id))
        .build()
        .unwrap();

      runtime.block_on(async {
        loop {
          let locked_reciever = reciever.lock().unwrap();
          let job = match locked_reciever.recv() {
            Ok(ok) => ok,
            Err(err) => {
              panic!("unknown error: {:#?}", err);
            }
          };

          println!("Worker {id} got a job; executing");

          job().await;
        }
      });
    });
    Worker { id, handle }
  }
}

pub struct ThreadPool {
  pool: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static>;

impl ThreadPool {
  /// Create a threadpool with n threads.
  ///
  /// # Panics
  /// Panics if size of threadpool is < 0.
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let (sender, reciever) = mpsc::channel();

    let reciever = Arc::new(Mutex::new(reciever));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&reciever)));
    }

    Self { pool: workers, sender }
  }

  pub fn execute(
    &self,
    f: Job,
    // f: Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static>,
  ) {
    // let job = Box::new(move || Box::pin(f()));

    self.sender.send(f).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  async fn hello() {
    println!("hello");
  }

  #[test]
  fn threadpool() {
    let pool = ThreadPool::new(5);

    pool.execute(Box::new(|| Box::pin(hello())));
    pool.execute(Box::new(|| Box::pin(hello())));
    pool.execute(Box::new(|| Box::pin(hello())));
    pool.execute(Box::new(|| Box::pin(hello())));
  }
}
