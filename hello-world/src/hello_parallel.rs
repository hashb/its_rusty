use std::thread;

pub fn hello_parallel() {
    for _ in 0..10 {
        thread::spawn(|| {
            println!("Hello, World!");
        });
    }
}
