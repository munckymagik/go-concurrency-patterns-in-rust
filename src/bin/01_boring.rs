//! Based on Go example
//! [slide 12: "A boring function"](https://talks.golang.org/2012/concurrency.slide#12)
//!
//! We need an example to show the interesting properties of the concurrency primitives.
//!
//! To avoid distraction, we make it a boring example.
//!
use std::thread;
use std::time::Duration;

mod helpers;

fn main() {
    boring("boring!");
}

fn boring(msg: &str) {
    for i in 0.. {
        println!("{} {}", msg, i);
        thread::sleep(Duration::from_secs(1));
    }
}
