use bytes::{BytesMut, BufMut};
use rand::distributions::Uniform;
use rand::Rng;
use crate::color::Color;


pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  _unused: f32,
  //pub r: f32,
  //pub g: f32,
  //pub b: f32,
  //pub a: f32,
  pub r: u32,
  pub g: u32,
  pub b: u32,
  pub a: u32,
}

impl Point {

  pub fn at(x: f32, y: f32, z: f32, color: Color) -> Self {
    Point {
      x,
      y,
      z,
      _unused: 0.0f32,
      //r: color.get_f32_r(),
      //g: color.get_f32_g(),
      //b: color.get_f32_b(),
      //a: 1.0f32,
      r: color.get_u32_r(),
      g: color.get_u32_g(),
      b: color.get_u32_b(),
      a: 1,
    }
  }

  pub fn at_random_range(min: f32, max: f32, color: Color) -> Self {
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(min, max);

    let x = rng.sample(&range);
    let y = rng.sample(&range);
    let z = rng.sample(&range);

    Point {
      x,
      y,
      z,
      _unused: 0.0f32,
      //r: color.get_f32_r(),
      //g: color.get_f32_g(),
      //b: color.get_f32_b(),
      //a: 1.0f32,
      r: color.get_u32_r(),
      g: color.get_u32_g(),
      b: color.get_u32_b(),
      a: 1,
    }
  }


  pub fn to_bytes(&self) -> Vec<u8> {
    let mut buf = BytesMut::with_capacity(32);
    buf.put_f32(self.x);
    buf.put_f32(self.y);
    buf.put_f32(self.z);
    buf.put_f32(0.0f32); // Unused
    //buf.put_f32(self.r);
    //buf.put_f32(self.g);
    //buf.put_f32(self.b);
    //buf.put_f32(self.a);
    buf.put_u32(self.r);
    buf.put_u32(self.g);
    buf.put_u32(self.b);
    buf.put_u32(self.a);
    buf.to_vec()
  }

  pub fn location_string(&self) -> String {
    format!("{}, {}, {}", self.x, self.y, self.z)
  }

  pub fn debug_string(&self) -> String {
    format!("(location: {}, {}, {}); (color: {}, {}, {}, alpha: {})",
            self.x, self.y, self.z,
            self.r, self.g, self.b, self.a)
  }
}