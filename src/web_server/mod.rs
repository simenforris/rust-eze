pub mod http_request;
pub mod http_response;
pub mod thread_pool;

use std::{
  io::prelude::*,
  net::{TcpListener, TcpStream},
};

use anyhow::{anyhow, Result};
use http_request::HttpRequest;
use http_response::HttpResponse;

use crate::web_server::thread_pool::ThreadPool;

/**
 * HTTP requst:
 *
 * <method> <uri> <HTTP-Version>\n
 * <...headers>\n
 * \n
 * <body>
 */

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
  let http_request = HttpRequest::from_tcp_stream(&stream);

  match http_request {
    Ok(request) => {
      println!("\nincoming http request: {}", request);

      let response = HttpResponse::default();

      println!("created new response: {}", response);

      return stream.write_all(&response.to_bytes());
    }
    Err(err) => {
      let mut response = HttpResponse::default();
      response.status = 400;
      response.reason = "Bad request".to_owned();
      response.body = Some(err.to_string());
      let response_bytes = response.to_bytes();
      return stream.write_all(&response_bytes);
    }
  }
}

pub fn init(port: u16, threads: Option<usize>) -> Result<()> {
  let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string()))?;
  let pool = ThreadPool::new(threads.unwrap_or(4));

  println!("\nServer listening on port {}", port.to_string());

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => pool.execute(|| {
        if let Err(err) = handle_connection(stream) {
          println!("failed to handle connection: {}", anyhow!(err))
        }
      }),
      Err(err) => println!("connection failed: {}", anyhow!(err)),
    }
  }

  return Ok(());
}
