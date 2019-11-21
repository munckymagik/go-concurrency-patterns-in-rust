use async_std::task;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    task::spawn(async { boring() });
    println!("I'm listening ...");
    helpers::sleep(2000);
    println!("You're boring; I'm leaving.");
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        helpers::sleep(thread_rng().gen_range(0, 1000));
    }
}
