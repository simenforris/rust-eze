use anyhow::{anyhow, Error};
use std::{
  collections::HashMap,
  fmt::Display,
  io::{prelude::*, BufReader},
  net::TcpStream,
  str::FromStr,
};

#[derive(Debug)]
pub enum HttpMethod {
  Get,
  Post,
  Put,
  Delete,
  Purge,
}

impl Display for HttpMethod {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      HttpMethod::Get => write!(f, "GET"),
      HttpMethod::Post => write!(f, "POST"),
      HttpMethod::Put => write!(f, "PUT"),
      HttpMethod::Delete => write!(f, "DELETE"),
      HttpMethod::Purge => write!(f, "PURGE"),
    }
  }
}

impl FromStr for HttpMethod {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_ascii_uppercase().as_str() {
      "GET" => Ok(HttpMethod::Get),
      "POST" => Ok(HttpMethod::Post),
      "PUT" => Ok(HttpMethod::Put),
      "DELETE" => Ok(HttpMethod::Delete),
      "PURGE" => Ok(HttpMethod::Purge),
      _ => Err(anyhow!("unknown http method")),
    }
  }
}

pub struct HttpRequest {
  pub method: HttpMethod,
  pub uri: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Display for HttpRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return write!(
      f,
      "HttpRequest({} {}) {:#?}",
      self.method, self.uri, self.headers
    );
  }
}

fn extract_request_line(line: String) -> Result<(HttpMethod, String, String), Error> {
  let mut it = line.split_whitespace();
  match (it.next(), it.next(), it.next()) {
    (Some(method), Some(uri), Some(version)) => Ok((
      HttpMethod::from_str(method)?,
      uri.to_string(),
      version.to_string(),
    )),
    _ => Err(anyhow!("badly formatted request line")),
  }
}

impl HttpRequest {
  pub fn from_tcp_stream(stream: &TcpStream) -> Result<Self, Error> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    let bytes = reader
      .read_line(&mut request_line)
      .map_err(|err| anyhow!(err))?;
    if bytes == 0 {
      return Err(anyhow!("empty request"));
    }
    let request_line = request_line.trim_end_matches(&['\r', '\n'][..]).to_string();

    let (method, uri, _version) = extract_request_line(request_line)?;

    let mut headers: HashMap<String, String> = HashMap::default();
    loop {
      let mut line = String::new();
      let bytes = reader.read_line(&mut line).map_err(|err| anyhow!(err))?;
      if bytes == 0 {
        break;
      }

      let line = line.trim_end_matches(&['\r', '\n'][..]);
      if line.is_empty() {
        break;
      }

      if let Some((name, value)) = line.split_once(':') {
        headers.insert(
          name.trim().to_ascii_lowercase().to_string(),
          value.trim().to_string(),
        );
      }
    }

    let content_length = headers
      .get("content-length")
      .and_then(|v| v.parse::<usize>().ok())
      .unwrap_or(0);

    let body = if content_length == 0 {
      None
    } else {
      let mut buf = vec![0u8; content_length];
      reader.read_exact(&mut buf).map_err(|err| anyhow!(err))?;
      Some(String::from_utf8_lossy(&buf).to_string())
    };

    return Ok(HttpRequest {
      method,
      uri,
      headers,
      body,
    });
  }

  pub fn path(&self) -> &str {
    return self
      .uri
      .split_once("?")
      .map(|(p, _)| p)
      .unwrap_or("/".into());
  }

  pub fn query_str(&self) -> Option<&str> {
    return self.uri.split_once("?").map(|(_, q)| q);
  }

  pub fn query(&self) -> HashMap<&str, &str> {
    let mut out = HashMap::new();

    if let Some(q) = self.query_str() {
      for pair in q.split("&").filter(|p| !p.is_empty()) {
        let (k, v) = pair.split_once("=").unwrap_or((pair, ""));
        out.insert(k, v);
      }
    }

    return out;
  }
}
