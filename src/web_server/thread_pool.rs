use std::{
  sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
  },
  thread::{Builder, JoinHandle},
};

use anyhow::Result;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Self> {
    let builder = Builder::new();

    let thread = builder.spawn(move || loop {
      let job = receiver
        .lock()
        .map_err(|err| format!("mutex lock failed {err}"))
        .and_then(|receiver| {
          receiver
            .recv()
            .map_err(|err| format!("failed to recv message {err}"))
        });

      match job {
        Ok(job) => {
          println!("Worker {id} got a job; executing");
          job();
        }
        Err(err) => println!("Worker {id} {err}"),
      }
    })?;

    return Ok(Self { id, thread });
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Job>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let (sender, receiver) = channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      if let Ok(worker) = Worker::new(id, Arc::clone(&receiver)) {
        workers.push(worker);
      }
    }

    return ThreadPool { workers, sender };
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    if let Err(err) = self.sender.send(job) {
      println!("failed to handle connection: {}", err)
    }
  }
}
