use async_std::task;
use std::thread;
use std::time::Duration;

mod helpers;

fn main() {
    task::spawn(boring());
    println!("I'm listening ...");
    thread::sleep(Duration::from_millis(2000));
    println!("You're boring; I'm leaving.");
}

async fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        task::sleep(helpers::rand_duration(0, 1000)).await;
    }
}
