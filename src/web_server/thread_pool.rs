use std::{
  sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
  },
  thread::{Builder, JoinHandle},
};

use anyhow::{anyhow, Result};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Self> {
    let builder = Builder::new();

    let thread = builder.spawn(move || loop {
      let message = receiver
        .lock()
        .map_err(|err| anyhow!("mutex lock failed {err}"))
        .and_then(|receiver| {
          receiver
            .recv()
            .map_err(|err| anyhow!("failed to recv message {err}"))
        });

      match message {
        Ok(job) => {
          println!("Worker {id} got a job; executing");
          job();
        }
        Err(err) => {
          println!("Worker {id} exited; shutting down. {err}");
          break;
        }
      }
    })?;

    return Ok(Self { id, thread });
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<Sender<Job>>,
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

    return ThreadPool {
      workers,
      sender: Some(sender),
    };
  }

  pub fn execute<F>(&self, f: F) -> Result<()>
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    return match self.sender.as_ref() {
      Some(sender) => sender
        .send(job)
        .map_err(|err| anyhow!("unable to pass job to worker {}", err)),
      None => Err(anyhow!("unable to pass job to worker, no sender")),
    };
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());

    for worker in &mut self.workers.drain(..) {
      println!("shutting down worker {}", worker.id);

      worker.thread.join().unwrap();
    }
  }
}
