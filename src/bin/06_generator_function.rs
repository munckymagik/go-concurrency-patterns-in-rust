//! Based on Go example
//! [slide 25: "Generator: function that returns a channel"](https://talks.golang.org/2012/concurrency.slide#25)
//!
//! The `Receiver` instance for a channel can be returned independently from the `Sender`.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

mod helpers;

fn main() {
    let mut receiver = boring("boring!"); // function returning a channel

    task::block_on(async {
        for _ in 0i32..5 {
            println!(
                "You say: {}",
                receiver.next().await.expect("Receiving failed")
            );
        }
    });

    println!("You're boring; I'm leaving.");
}

fn boring(message: &str) -> Receiver<String> {
    // returns a receiver for a channel of Strings
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    // we launch an async task from within the function
    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            sender
                .send(msg)
                .await
                .expect("Failed to send message to channel");
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    receiver // return the receiver to the caller
}
