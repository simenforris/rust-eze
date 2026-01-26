pub mod area;
pub mod circle;
pub mod collision;
pub mod rectangle;
pub mod shape;

use crate::shapes::{collision::Collidable, shape::Shape};
use std::fs::read_to_string;

use anyhow::Result;

pub fn read_shapes() -> Result<()> {
  let shapes = read_to_string("shapes")?
    .lines()
    .filter_map(|x| x.parse::<Shape>().ok())
    .collect::<Vec<_>>();

  shapes
    .iter()
    .skip(1)
    .zip(shapes.iter().take(shapes.len() - 1))
    .filter(|(a, b)| a.collide(b))
    .for_each(|(a, b)| {
      println!("{} collides with {}", a, b);
    });

  return Ok(());
}
