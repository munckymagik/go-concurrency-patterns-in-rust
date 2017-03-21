extern crate rand;

use std::{thread, time};
use rand::{thread_rng, Rng};

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        sleep(thread_rng().gen_range(0, 1000));
    };
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
