//! Also  based on Go example
//! [slide 36: "Timeout for whole conversation using select"](https://talks.golang.org/2012/concurrency.slide#36),
//! however this example demonstrates the same pattern using the
//! [async_std::stream::Stream::timeout](https://docs.rs/async-std/1.2.0/async_std/stream/trait.Stream.html#method.timeout)
//! combinator.
use async_std::stream::{StreamExt, TimeoutError};
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use std::time;

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let duration = time::Duration::from_millis(5000);
    let mut c = boring("Joe").timeout(duration);

    while let Some(item) = c.next().await {
        match item {
            Ok(s) => println!("{}", s),
            Err(TimeoutError { .. }) => {
                println!("You talk too much.");
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
