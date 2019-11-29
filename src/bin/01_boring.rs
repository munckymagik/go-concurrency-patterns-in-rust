use std::thread;
use std::time::Duration;

mod helpers;

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        thread::sleep(Duration::from_millis(500));
    }
}
