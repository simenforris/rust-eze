use std::{fmt::Display, str::FromStr};

use super::{
  circle::Circle,
  collision::{Contains, PointIter, Points},
  rectangle::Rectangle,
};

use anyhow::{anyhow, Error};

pub enum Shape {
  Circle(Circle),
  Rectangle(Rectangle),
}

impl Points for &Shape {
  fn points(&self) -> PointIter {
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
