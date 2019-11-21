use rand::{thread_rng, Rng};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

mod helpers;

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
            helpers::sleep(thread_rng().gen_range(0, 1000));
        }
    });

    rx
}
