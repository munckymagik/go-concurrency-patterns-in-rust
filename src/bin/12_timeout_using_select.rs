#[macro_use]
extern crate chan;
extern crate rand;

use std::{thread, time};
use chan::Receiver;
use rand::{thread_rng, Rng};

fn main() {
    let c = boring("Joe");
    let duration = time::Duration::from_millis(500);

    loop {
        // In each loop, Joe has up to 500 ms to respond or the programme times-out.
        let timeout = chan::after(duration);
        chan_select! {
            c.recv() -> s => println!("{}", s.unwrap()),
            timeout.recv() => {
                println!("You're too slow.");
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
