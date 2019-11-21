use rand::{thread_rng, Rng};
use std::{thread, time};

fn main() {
    thread::spawn(|| boring());
    println!("I'm listening ...");
    sleep(2000);
    println!("You're boring; I'm leaving.");
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        sleep(thread_rng().gen_range(0, 1000));
    }
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
