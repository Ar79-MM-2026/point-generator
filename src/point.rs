
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  _unused: f32,
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Point {

  pub fn white_at(x: f32, y: f32, z: f32) -> Self {
    Point {
      x,
      y,
      z,
      _unused: 0.0f32,
      r: 1.0f32,
      g: 1.0f32,
      b: 1.0f32,
      a: 1.0f32,
    }
  }
}