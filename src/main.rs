mod shapes;
mod web_server;

use anyhow::Result;

fn main() -> Result<()> {
  web_server::init(8080);

  return Ok(());
}
