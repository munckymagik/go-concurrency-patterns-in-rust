use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use rand::{thread_rng, Rng};
use std::thread;

mod helpers;

fn main() {
    let quit_channel = bounded(0);
    // Using `ref` to prevent move out of `quit_channel` before we pass to `boring`
    let (ref quit_tx, ref quit_rx) = quit_channel;

    let c = boring("Joe", quit_channel.clone());

    // The loop will iterate printing Joe's messages until the overall timeout occurs.
    for _ in 0..(thread_rng().gen_range(1, 10)) {
        println!("{}", c.recv().unwrap());
    }

    quit_tx
        .send("Bye!".to_owned())
        .expect("sending quit failed");
    println!("Joe said: {}", quit_rx.recv().unwrap());
}

fn boring(message: &str, quit_channel: (Sender<String>, Receiver<String>)) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        let (quit_tx, quit_rx) = quit_channel;

        for i in 0.. {
            let msg_i = format!("{} {}", message_for_closure, i);
            select! {
                send(tx, msg_i) -> res => res.expect("sending failed"),
                recv(quit_rx) -> quit_msg => {
                    println!("main said: {}", quit_msg.unwrap());
                    quit_tx.send("See you!".to_owned()).unwrap();
                },
            }
            thread::sleep(helpers::rand_duration(0, 1000));
        }
    });

    rx
}
