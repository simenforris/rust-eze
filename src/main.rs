mod shapes;

use std::{fmt::Display, fs::read_to_string, str::FromStr};

use crate::shapes::{
  circle::Circle,
  collision::{Collidable, Contains, Points},
  rectangle::Rectangle,
};
use anyhow::{anyhow, Error, Result};

enum Shape {
  Circle(Circle),
  Rectangle(Rectangle),
}

impl Points for &Shape {
  fn points(&self) -> shapes::collision::PointIter {
    match self {
      Shape::Circle(c) => c.points(),
      Shape::Rectangle(r) => r.points(),
    }
  }
}

impl Contains for &Shape {
  fn contains_point(&self, point: (f64, f64)) -> bool {
    match self {
      Shape::Circle(c) => c.contains_point(point),
      Shape::Rectangle(r) => r.contains_point(point),
    }
  }
}

impl FromStr for Shape {
  type Err = Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

    match shape {
      "rect" => return Ok(Self::Rectangle(data.parse()?)),
      "circle" => return Ok(Self::Circle(data.parse()?)),
      _ => return Err(anyhow!("bad shape")),
    }
  }
}

impl Display for &Shape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Shape::Circle(c) => return write!(f, "{}", c),
      Shape::Rectangle(r) => return write!(f, "{}", r),
    }
  }
}

fn main() -> Result<()> {
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
