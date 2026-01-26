mod shapes;

use anyhow::Result;

use crate::shapes::read_shapes;

fn main() -> Result<()> {
  read_shapes()?;

  return Ok(());
}
