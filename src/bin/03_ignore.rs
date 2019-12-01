//! Based on Go example
//! [slide 15: "Ignoring it"](https://talks.golang.org/2012/concurrency.slide#15)
//!
//! The [task::spawn](https://docs.rs/async-std/1.2.0/async_std/task/fn.spawn.html)
//! function runs `boring` as usual, but doesn't make the caller wait.
//! It launches and runs `boring` in an asynchronous task.
//!
//! The functionality is analogous to the & on the end of a shell command.
//!
use async_std::task;

mod helpers;

fn main() {
    task::spawn(boring("boring!"));
    println!("I got bored");
}

async fn boring(msg: &str) {
    for i in 0.. {
        println!("{} {}", msg, i);
        task::sleep(helpers::rand_duration(0, 1000)).await;
    }
}
