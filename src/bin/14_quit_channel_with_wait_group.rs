// Using a WaitGroup keeps main alive so we can use stdout in Joe's quit handler

#[macro_use]
extern crate chan;


use std::{thread, time};
use chan::Receiver;
use rand::{thread_rng, Rng};

fn main() {
    let (quit_tx, quit_rx) = chan::sync(0);
    let (c, finished) = boring("Joe", quit_rx);

    // The loop will iterate printing Joe's messages until the overall timeout occurs.
    for _ in 0..(thread_rng().gen_range(0, 10)) {
        println!("{}", c.recv().unwrap());
    }

    println!("main: Quit please ...");
    quit_tx.send(());
    finished.wait();
    println!("main: Good. I'm out of here.");
}

fn boring(message: &str, quit_rx: Receiver<()>) -> (Receiver<String>, chan::WaitGroup) {
    let message_for_closure = message.to_owned();
    let (tx, rx) = chan::r#async();
    let wg = chan::WaitGroup::new();
    let wg_for_closure = wg.clone();
    wg.add(1);

    thread::spawn(move || {
        for i in 0.. {
            let msg_i = format!("{} {}", message_for_closure, i);
            chan_select! {
                tx.send(msg_i) => { /* do nothing */ },
                quit_rx.recv() => {
                    println!("{}: Oh ok, I'll quit", message_for_closure);
                    wg_for_closure.done();
                    return
                },
            }
            sleep(thread_rng().gen_range(0, 1000));
        };
    });


    (rx, wg)
}

fn sleep(dur_ms: u64) {
    thread::sleep(time::Duration::from_millis(dur_ms));
}
