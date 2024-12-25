pub trait Positionable {
  fn x(&self) -> f32;
  fn y(&self) -> f32;
}

pub struct Vector2D {
  pub x: f32,
  pub y: f32
}

impl Positionable for Vector2D {
  fn x(&self) -> f32 {
    self.x
  }

  fn y(&self) -> f32 {
    self.y
  }
}