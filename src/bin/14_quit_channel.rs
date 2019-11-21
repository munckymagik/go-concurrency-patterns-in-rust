#[macro_use]
extern crate chan;

use chan::Receiver;
use rand::{thread_rng, Rng};
use std::thread;

mod helpers;

fn main() {
    let (quit_tx, quit_rx) = chan::sync(0);
    let c = boring("Joe", quit_rx);

    // The loop will iterate printing Joe's messages until the overall timeout occurs.
    for _ in 0..(thread_rng().gen_range(0, 10)) {
        println!("{}", c.recv().unwrap());
    }

    println!("quitting ...");
    quit_tx.send(());
}

fn boring(message: &str, quit_rx: Receiver<()>) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = chan::r#async();

    thread::spawn(move || {
        for i in 0.. {
            let msg_i = format!("{} {}", message_for_closure, i);
            chan_select! {
                tx.send(msg_i) => { /* do nothing */ },
                quit_rx.recv() => return,
            }
            helpers::sleep(thread_rng().gen_range(0, 1000));
        }
    });

    rx
}
