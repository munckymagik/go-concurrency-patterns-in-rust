use std::thread;

mod helpers;

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        thread::sleep(helpers::rand_duration(0, 1000));
    }
}
