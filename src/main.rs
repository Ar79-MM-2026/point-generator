mod point;

use point::Point;

fn main() {
  let ctx = zmq::Context::new();

  let socket = ctx.socket(zmq::REQ).unwrap();

  let point = Point::white_at(0.0f32, 0.0f32, 0.0f32);


  socket.connect("tcp://127.0.0.1:1234").unwrap();
  socket.send("hello world!", 0).unwrap();
}
