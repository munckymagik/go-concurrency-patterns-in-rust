

use std::{thread, time};
use std::sync::mpsc::{channel, Sender};
use rand::{thread_rng, Rng};

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || boring("boring!", tx));

    for _ in 0..5 {
        println!("You say: {}", rx.recv().unwrap());
    }
    println!("You're boring; I'm leaving.");
}

fn boring(msg: &str, tx: Sender<String>) {
    for i in 0.. {
        tx.send(format!("{} {}", msg, i))
            .expect("Failed to send message to channel");
        sleep(thread_rng().gen_range(0, 1000));
    };
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
