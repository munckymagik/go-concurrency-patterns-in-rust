//! Also, based on Go example
//! [slide 35: "Timeout using select"](https://talks.golang.org/2012/concurrency.slide#35),
//! however this example demonstrates the same pattern using the
//! [async_std::future::timeout](https://docs.rs/async-std/1.2.0/async_std/future/fn.timeout.html)
//! combinator.
//!
//! It wraps the future returned by `c.next()`, then awaits the delivery of a message or
//! it times out after the given duration of time.
//!
use async_std::future::{timeout, TimeoutError};
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::time;

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let mut c = boring("Joe");
    let duration = time::Duration::from_millis(500);

    loop {
        // In each loop, Joe has up to 500 ms to respond or the program times-out.
        match timeout(duration, c.next()).await {
            Ok(s) => println!("{}", s.unwrap()),
            Err(TimeoutError { .. }) => {
                println!("You're too slow.");
                return;
            }
        }
    }
}

fn boring(message: &str) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            sender.send(msg).await.expect("sending failed");
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    receiver
}
