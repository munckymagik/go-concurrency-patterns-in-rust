/*
 * Now using chan_select! from the chan crate in fan_in.
 */

#[macro_use]
extern crate chan;

use chan::{Receiver, Sender};
use rand::{thread_rng, Rng};
use std::thread;

mod helpers;

fn main() {
    let c = fan_in(boring("Joe"), boring("Ann"));

    for _ in 0..10 {
        // Retrieve 2 messages at a time from queue
        let msg1 = c.recv().expect("channel closed getting msg1");
        let msg2 = c.recv().expect("channel closed getting msg2");

        println!("{}", msg1.message);
        println!("{}", msg2.message);

        // Send the continuation messages. These block until the receiver reads.
        msg1.tx_continue.send(true);
        msg2.tx_continue.send(true);
    }
    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(input1: Receiver<T>, input2: Receiver<T>) -> Receiver<T> {
    let (tx, rx) = chan::r#async();

    thread::spawn(move || loop {
        chan_select! {
            input1.recv() -> msg => tx.send(msg.expect("msg from input1")),
            input2.recv() -> msg => tx.send(msg.expect("msg from input2")),
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
    let (tx, rx) = chan::r#async();

    // Sets the buffer size to 0 to create a 'rendezvous' channel.
    let (tx_continue, rx_continue) = chan::sync(0);

    thread::spawn(move || {
        for i in 0.. {
            let msg = Message {
                message: format!("{} {}", message_for_closure, i),
                tx_continue: tx_continue.to_owned(),
            };

            tx.send(msg);

            helpers::sleep(thread_rng().gen_range(0, 1000));

            // Block here until the receiver has sent a continuation message
            rx_continue.recv().expect("boring wait");
        }
    });

    rx
}
