use rand::{thread_rng, Rng};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

mod helpers;

fn main() {
    let c = fan_in(boring("Joe"), boring("Ann"));

    for _ in 0..10 {
        match c.recv() {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{}", err),
        };
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
