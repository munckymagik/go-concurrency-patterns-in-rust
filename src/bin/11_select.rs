use async_std::task;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

mod helpers;

fn main() {
    let mut c = fan_in(boring("Joe"), boring("Ann"));

    task::block_on(async {
        for i in 0i32..10 {
            // Retrieve 2 messages at a time from queue
            let mut msg1 = c.next().await.expect("msg1");
            let mut msg2 = c.next().await.expect("msg2");

            println!("{}", msg1.message);
            println!("{}", msg2.message);

            if i < 9 {
                // Send the continuation messages.
                msg1.tx_continue.send(true).await.expect("msg1");
                msg2.tx_continue.send(true).await.expect("msg2");
            }
        }
    });

    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(mut input1: Receiver<T>, mut input2: Receiver<T>) -> Receiver<T> {
    let (mut tx, rx) = channel(0);

    task::spawn(async move {
        loop {
            select! {
                msg = input1.select_next_some() => tx.send(msg).await.expect("send1 in fan_in"),
                msg = input2.select_next_some() => tx.send(msg).await.expect("send2 in fan_in"),
            }
        }
    });

    rx
}

struct Message {
    /// The message text to send
    message: String,
    /// The producing thread will block until the receiving thread sends a continuation message
    /// over this channel
    tx_continue: Sender<bool>,
}

fn boring(message: &str) -> Receiver<Message> {
    let message_for_closure = message.to_owned();
    let (mut tx, rx) = channel(0);

    // Sets the buffer size to 0 to create a 'rendezvous' channel.
    let (tx_continue, mut rx_continue) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = Message {
                message: format!("{} {}", message_for_closure, i),
                tx_continue: tx_continue.to_owned(),
            };

            tx.send(msg).await.expect("boring send");

            task::sleep(helpers::rand_duration(0, 1000)).await;

            // Pause here until the receiver has sent a continuation message
            rx_continue.next().await.expect("boring wait");
        }
    });

    rx
}
