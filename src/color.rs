#[derive(Copy, Clone, Debug)]
pub enum Color {
  Black,
  White,
  Red,
  Green,
  Blue,
  Custom { r: f32, g: f32, b: f32 },
}

impl Color {
  pub fn get_f32_r(&self) -> f32 {
    match &self {
      Color::Black => 0.0f32,
      Color::White => 1.0f32,
      Color::Red => 1.0f32,
      Color::Green => 0.0f32,
      Color::Blue => 0.0f32,
      Color::Custom {r, g, b} => *r,
    }
  }

  pub fn get_f32_g(&self) -> f32 {
    match &self {
      Color::Black => 0.0f32,
      Color::White => 1.0f32,
      Color::Red => 0.0f32,
      Color::Green => 1.0f32,
      Color::Blue => 0.0f32,
      Color::Custom {r, g, b} => *g,
    }
  }

  pub fn get_f32_b(&self) -> f32 {
    match &self {
      Color::Black => 0.0f32,
      Color::White => 1.0f32,
      Color::Red => 1.0f32,
      Color::Green => 0.0f32,
      Color::Blue => 0.0f32,
      Color::Custom {r, g, b} => *b,
    }
  }
}
