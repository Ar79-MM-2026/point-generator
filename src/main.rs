//! Generate random points to send over ZeroMQ to a consumer

mod point;
mod color;

use point::Point;
use std::thread;
use zmq::Error;
use std::time::Duration;
use crate::color::Color;

fn main() {
  let ctx = zmq::Context::new();

  let socket = ctx.socket(zmq::REQ).unwrap();

  socket.connect("tcp://127.0.0.1:1234").unwrap();
  //socket.send("hello world!", 0).unwrap();

  loop {
    let point = Point::at_random_range(-1000.0f32, 1000.0f32, Color::White);

    println!("Location: {}", point.location_string());

    let bytes = point.to_bytes();

    //let result = socket.send("hello world!", 0);
    let result = socket.send(&bytes, 0);

    match result {
      Ok(_) => continue,
      Err(e) => {
        eprintln!("Error: {:?}", e);
        thread::sleep(Duration::from_millis(5_000));
      },
    }
  }
}
