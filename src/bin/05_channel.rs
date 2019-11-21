use rand::{thread_rng, Rng};
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod helpers;

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
        helpers::sleep(thread_rng().gen_range(0, 1000));
    }
}
