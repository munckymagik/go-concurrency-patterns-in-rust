use rand::{thread_rng, Rng};
use std::thread;

mod helpers;

fn main() {
    thread::spawn(|| boring());
    println!("I got bored");
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        helpers::sleep(thread_rng().gen_range(0, 1000));
    }
}
