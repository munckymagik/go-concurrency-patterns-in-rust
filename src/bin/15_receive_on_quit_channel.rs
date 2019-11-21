#[macro_use]
extern crate chan;


use std::{thread, time};
use chan::{Sender, Receiver};
use rand::{thread_rng, Rng};

fn main() {
    let quit_channel = chan::sync(0);
    // Using `ref` to prevent move out of `quit_channel` before we pass to `boring`
    let (ref quit_tx, ref quit_rx) = quit_channel;

    let c = boring("Joe", quit_channel.clone());

    // The loop will iterate printing Joe's messages until the overall timeout occurs.
    for _ in 0..(thread_rng().gen_range(0, 10)) {
        println!("{}", c.recv().unwrap());
    }

    quit_tx.send("Bye!".to_owned());
    println!("Joe said: {}", quit_rx.recv().unwrap());
}

fn boring(message: &str, quit_channel: (Sender<String>, Receiver<String>)) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = chan::r#async();

    thread::spawn(move || {
        let (quit_tx, quit_rx) = quit_channel;

        for i in 0.. {
            let msg_i = format!("{} {}", message_for_closure, i);
            chan_select! {
                tx.send(msg_i) => { /* do nothing */ },
                quit_rx.recv() -> quit_msg => {
                    println!("main said: {}", quit_msg.unwrap());
                    quit_tx.send("See you!".to_owned());
                },
            }
            sleep(thread_rng().gen_range(0, 1000));
        };
    });

    rx
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
