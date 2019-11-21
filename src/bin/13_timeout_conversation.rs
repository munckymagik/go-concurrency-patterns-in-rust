#[macro_use]
extern crate chan;

use chan::Receiver;
use rand::{thread_rng, Rng};
use std::{thread, time};

mod helpers;

fn main() {
    let c = boring("Joe");
    let timeout = chan::after(time::Duration::from_secs(5));

    // The loop will iterate printing Joe's messages until the overall timeout occurs.
    loop {
        chan_select! {
            c.recv() -> s => println!("{}", s.unwrap()),
            timeout.recv() => {
                println!("You talk too much.");
                return;
            },
        }
    }
}

fn boring(message: &str) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = chan::r#async();

    thread::spawn(move || {
        for i in 0.. {
            tx.send(format!("{} {}", message_for_closure, i));
            helpers::sleep(thread_rng().gen_range(0, 1000));
        }
    });

    rx
}
