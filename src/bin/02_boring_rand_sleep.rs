//! Based on Go example
//! [slide 13: "Slightly less boring"](https://talks.golang.org/2012/concurrency.slide#13)
//!
//! Make the intervals between messages unpredictable (still under a second).
//!
use std::thread;

mod helpers;

fn main() {
    boring("boring!");
}

fn boring(msg: &str) {
    for i in 0.. {
        println!("{} {}", msg, i);
        thread::sleep(helpers::rand_duration(0, 1000));
    }
}
