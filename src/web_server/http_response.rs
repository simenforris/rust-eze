use std::{collections::HashMap, fmt::Display};

pub struct HttpResponse {
  pub status: u16,
  pub reason: String,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Default for HttpResponse {
  fn default() -> Self {
    Self {
      status: 200,
      reason: "OK".into(),
      headers: Default::default(),
      body: Some("OK".into()),
    }
  }
}

impl Display for HttpResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "HttpResponse({} {})",
      self.status.to_string(),
      self.reason,
    )
  }
}

impl HttpResponse {
  pub fn as_string(&self) -> String {
    return format!(
      "HTTP/1.1 {} {}\r\n{}\r\n{}",
      self.status.to_string(),
      self.reason,
      self
        .headers
        .iter()
        .map(|(k, v)| format!("{}: {}\r\n", k, v))
        .collect::<String>(),
      self.body.as_ref().map_or("", |v| v)
    );
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    self.as_string().into_bytes()
  }
}
