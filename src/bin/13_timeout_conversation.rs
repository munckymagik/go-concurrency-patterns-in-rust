#[macro_use]
extern crate chan;
extern crate rand;

use std::{thread, time};
use chan::Receiver;
use rand::{thread_rng, Rng};

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
            sleep(thread_rng().gen_range(0, 1000));
        };
    });

    rx
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
