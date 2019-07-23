use std::thread;

fn main() {
  for _ in 0..10 {
    thread::spawn(|| {
      println!("Hello, World!");
    });
  }
}

