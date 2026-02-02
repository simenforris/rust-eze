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

use crate::web_server::{http_request::HttpMethod, thread_pool::ThreadPool};

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
      println!("\n--- incoming http request ---\n{}\n", request);

      let response: Result<HttpResponse> = match (&request.method, request.path()) {
        (HttpMethod::Get, "/sleep") => Ok(HttpResponse::default()),
        _ => {
          let mut not_found = HttpResponse::default();
          not_found.status = 404;
          not_found.reason = "Not Found".into();
          Ok(not_found)
        }
      };

      return match response {
        Ok(response) => stream.write_all(&response.to_bytes()),
        Err(err) => {
          let mut internal_error = HttpResponse::default();
          internal_error.status = 500;
          internal_error.reason = "Internal Server Error".into();
          internal_error.body = Some(err.to_string());
          return stream.write_all(&internal_error.to_bytes());
        }
      };
    }
    Err(_err) => {
      let mut response = HttpResponse::default();
      response.status = 400;
      response.reason = "Bad request".to_owned();
      response.body = None;
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
      })?,
      Err(err) => println!("connection failed: {}", anyhow!(err)),
    }
  }

  println!("\nServer shutting down");

  return Ok(());
}
