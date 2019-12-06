//! Also based on Go example
//! [slide 39: "Daisy-chain"](https://talks.golang.org/2012/concurrency.slide#39),
//! however this time we use async-std's "green threads" style
//! [tasks](https://docs.rs/async-std/1.2.0/async_std/task/index.html).
//!
//! This previous example demonstrated the limits of using threads. However,
//! using asynchronous tasks we no longer have a limit placed on the length of
//! the chain (other than probably your system memory). Now we are able to
//! increase the number of links to 10,000.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

async fn f(mut left: Sender<i64>, mut right: Receiver<i64>) {
    let val = right.next().await.expect("receiver problem");
    left.send(val + 1).await.expect("receiver hung up already");
}

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let n = 10_000; // Hurray! We are no longer constrained by the OS max threads limit

    // We will receive the final count from the leftmost_receiver. As we build
    // the chain rightmost_sender will be continually updated to point to the
    // furthest front of the chain, until finally it points to the start of the
    // chain.
    let (mut rightmost_sender, mut leftmost_receiver) = channel(0);

    for _ in 1..n {
        // Create a channel. This will form the connection between one link in
        // the chain and the next.
        let (next_sender, this_receiver) = channel(0);

        // Create a task for this link in the chain.
        task::spawn(f(rightmost_sender, this_receiver));

        // Update rightmost_sender to point to the front of the chain so we can
        // connect it to the next link in the chain in the next iteration of
        // the loop.
        rightmost_sender = next_sender;
    }

    // Start passing the message through the chain
    task::spawn(async move { rightmost_sender.send(1).await });

    // Await then print the final value from the chain
    println!(
        "{}",
        leftmost_receiver
            .next()
            .await
            .expect("receiving final value")
    );
}
