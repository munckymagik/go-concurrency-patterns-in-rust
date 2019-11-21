/*
 * Note in this implementation the producers do not swap around e.g.:
 *
 * Joe 0
 * Ann 0
 * Joe 1
 * Ann 1
 * Joe 2
 * Ann 2
 *
 * Because the continuation `send` calls DO block it is NOT possible for the second thread to jump
 * ahead and be the next to post a message.
 */



use std::{thread, time};
use std::sync::mpsc::{channel, sync_channel, Receiver, SyncSender};
use rand::{thread_rng, Rng};

fn main() {
    let c = fan_in(boring("Joe"), boring("Ann"));

    for _ in 0..10 {
        // Retrieve 2 messages at a time from queue
        let msg1 = c.recv().expect("msg1");
        let msg2 = c.recv().expect("msg2");

        println!("{}", msg1.message);
        println!("{}", msg2.message);

        // Send the continuation messages. These block until the receiver reads.
        msg1.tx_continue.send(true).expect("msg1");
        msg2.tx_continue.send(true).expect("msg2");
    }
    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(input1: Receiver<T>, input2: Receiver<T>) -> Receiver<T> {
    let (tx, rx) = channel();
    let tx2 = tx.clone();

    thread::spawn(move || loop {
                      let msg = input1.recv().expect("input1 recv failed");
                      tx.send(msg).expect("input1 send failed");
                  });
    thread::spawn(move || loop {
                      let msg = input2.recv().expect("input2 recv failed");
                      tx2.send(msg).expect("input2 send failed");
                  });

    rx
}

struct Message {
    /// The message text to send
    message: String,
    /// The producing thread will block until the receiving thread sends a continuation message
    /// over this channel
    tx_continue: SyncSender<bool>,
}

fn boring(message: &str) -> Receiver<Message> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = channel();

    // Sets the buffer size to 0 to create a 'rendezvous' channel.
    let (tx_continue, rx_continue) = sync_channel(0);

    thread::spawn(move || {
        for i in 0.. {
            let msg = Message {
                message: format!("{} {}", message_for_closure, i),
                tx_continue: tx_continue.to_owned(),
            };

            tx.send(msg).expect("boring send");

            sleep(thread_rng().gen_range(0, 1000));

            // Block here until the receiver has sent a continuation message
            rx_continue.recv().expect("boring wait");
        }
    });

    rx
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
