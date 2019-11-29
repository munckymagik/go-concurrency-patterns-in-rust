//! Based on Go example
//! [slide 39: "Daisy-chain"](https://talks.golang.org/2012/concurrency.slide#39)
//!
//! The idea is to chain a long sequence of tasks together, each one passing a
//! message to the next until the message finally emerges from the last
//! receiver in main.
//!
//! This first example demonstrates the limits of using threads for this
//! pattern, as the length of the chain is limited by the maximum number of
//! threads per process configured in your operating system.
//!
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

// The limit for this example seems to be the maximum number of threads per
// process, which on my Mac appears to be 2048 and is confirmed by the output
// of `sysctl kern.num_taskthreads`
const MAX_THREADS: usize = 2048;

fn f(left: SyncSender<i64>, right: Receiver<i64>) {
    left.send(right.recv().unwrap() + 1)
        .expect("sending failed in f");
}

fn main() {
    let n = MAX_THREADS - 1; // We need to deduct 1 to allow for the main thread

    // We will receive the final count from the leftmost_receiver. As we build
    // the chain rightmost_sender will be continually updated to point to the
    // furthest front of the chain, until finally it points to the start of the
    // chain.
    let (mut rightmost_sender, leftmost_receiver) = sync_channel(0);

    for _ in 1..n {
        // Create a channel. This will form the connection between one link in
        // the chain and the next.
        let (next_sender, this_receiver) = sync_channel(0);

        // Create a worker thread for this link in the chain.
        thread::spawn(move || f(rightmost_sender, this_receiver));

        // Update rightmost_sender to point to the front of the chain so we can
        // connect it to the next link in the chain in the next iteration of
        // the loop.
        rightmost_sender = next_sender;
    }

    // Start passing the message through the chain
    thread::spawn(move || rightmost_sender.send(1));

    // Await then print the final value from the chain
    println!("{}", leftmost_receiver.recv().unwrap());
}
