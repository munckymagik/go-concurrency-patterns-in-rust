//! Based on Go example
//! [slide 32: "Select"](https://talks.golang.org/2012/concurrency.slide#32)
//!
//! The [`select!` macro](https://docs.rs/futures/0.3.1/futures/macro.select.html)
//! provides a way to handle multiple futures or streams. Because channels are
//! based on streams we can use `select!` to handle our channels.
//!
//! It's like a switch, but each case is a communication:
//!
//! * All futures and streams are evaluated.
//! * Selection blocks until one communication can proceed, which then does.
//! * If multiple can proceed, select chooses pseudo-randomly.
//! * A default clause, if present, executes immediately if no channel is ready.
//!
//! We can rewrite our original `fan_in` function. Only one task is needed.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let mut c = fan_in(boring("Joe"), boring("Ann"));

    for i in 0i32..10 {
        // Retrieve 2 messages at a time from queue
        let mut msg1 = c.next().await.expect("msg1");
        let mut msg2 = c.next().await.expect("msg2");

        println!("{}", msg1.message);
        println!("{}", msg2.message);

        if i < 9 {
            // Send the continuation messages.
            msg1.sender_continue.send(true).await.expect("msg1");
            msg2.sender_continue.send(true).await.expect("msg2");
        }
    }

    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(mut input1: Receiver<T>, mut input2: Receiver<T>) -> Receiver<T> {
    let (mut sender, receiver) = channel(0);

    // Only one task is needed now we are using `select!`
    task::spawn(async move {
        loop {
            select! {
                msg = input1.select_next_some() => sender.send(msg).await.expect("send1 in fan_in"),
                msg = input2.select_next_some() => sender.send(msg).await.expect("send2 in fan_in"),
            }
        }
    });

    receiver
}

struct Message {
    /// The message text to send
    message: String,
    /// The producing thread will block until the receiving thread sends a continuation message
    /// over this channel
    sender_continue: Sender<bool>,
}

fn boring(message: &str) -> Receiver<Message> {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    // Sets the buffer size to 0 to create a 'rendezvous' channel.
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
