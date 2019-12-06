//! Based on Go example
//! [slide 29: "Restoring sequencing"](https://talks.golang.org/2012/concurrency.slide#29)
//!
//! Send a channel on a channel, making task wait its turn.
//!
//! Receive all messages, then enable them again by sending on a private channel.
//!
//! We define a message type `Message` that contains a channel for the reply.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

mod helpers;

/// A message type that contains a channel for the reply.
struct Message {
    /// The message text to send
    message: String,
    /// The producing task will wait until the receiving task sends a continuation message
    /// over this channel
    sender_continue: Sender<bool>,
}

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let mut c = fan_in(boring("Joe"), boring("Ann"));

    for _ in 0i32..10 {
        // Retrieve 2 messages at a time from queue
        let mut msg1 = c.next().await.expect("msg1");
        let mut msg2 = c.next().await.expect("msg2");

        println!("{}", msg1.message);
        println!("{}", msg2.message);

        // Send the continuation messages. Each speaker must wait for a go-ahead.
        msg1.sender_continue.send(true).await.expect("msg1");
        msg2.sender_continue.send(true).await.expect("msg2");
    }

    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(mut input1: Receiver<T>, mut input2: Receiver<T>) -> Receiver<T> {
    let (mut sender, receiver) = channel(0);
    let mut sender2 = sender.clone();

    task::spawn(async move {
        while let Some(msg) = input1.next().await {
            sender.send(msg).await.expect("input1 send failed");
        }
    });
    task::spawn(async move {
        while let Some(msg) = input2.next().await {
            sender2.send(msg).await.expect("input2 send failed");
        }
    });

    receiver
}

fn boring(message: &str) -> Receiver<Message> {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);
    let (sender_continue, mut receiver_continue) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = Message {
                message: format!("{} {}", message_for_closure, i),
                sender_continue: sender_continue.to_owned(),
            };

            sender.send(msg).await.expect("boring send");

            task::sleep(helpers::rand_duration(0, 1000)).await;

            // Pause here until the receiver has sent a continuation message
            receiver_continue.next().await.expect("boring wait");
        }
    });

    receiver
}
