extern crate rand;

use std::{thread, time};
use std::sync::mpsc::{channel, Receiver};
use rand::{thread_rng, Rng};

fn main() {
    let rx = boring("boring!");

    for _ in 0..5 {
        println!("You say: {}", rx.recv().unwrap());
    }
    println!("You're boring; I'm leaving.");
}

fn boring(message: &str) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = channel();

    thread::spawn(move || {
        for i in 0.. {
            tx.send(format!("{} {}", message_for_closure, i))
                .expect("Failed to send message to channel");
            sleep(thread_rng().gen_range(0, 1000));
        };
    });

    rx
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}