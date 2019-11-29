//! Based on Go example
//! [slide 16: "Ignoring it a little less"](https://talks.golang.org/2012/concurrency.slide#16)
//!
//! When main returns, the program exits and takes the boring function down with it.
//!
//! We can hang around a little, and on the way show that both main and the spawned task are running.
//!
use async_std::task;
use std::thread;
use std::time::Duration;

mod helpers;

fn main() {
    task::spawn(boring("boring!"));
    println!("I'm listening ...");
    thread::sleep(Duration::from_millis(2000));
    println!("You're boring; I'm leaving.");
}

async fn boring(msg: &str) {
    for i in 0.. {
        println!("{} {}", msg, i);
        task::sleep(helpers::rand_duration(0, 1000)).await;
    }
}
