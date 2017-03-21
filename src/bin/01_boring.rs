use std::{thread, time};

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        sleep(500);
    };
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
