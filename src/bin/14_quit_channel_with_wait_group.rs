// Using a WaitGroup keeps main alive so we can use stdout in Joe's quit handler
//
// Note that crossbeam's WaitGroup is not "Futures-aware" so it doesn't work
// with async/await, and it will block waiting to obtain a lock on it's internal
// Mutex.

use async_std::task;
use crossbeam_utils::sync::WaitGroup;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::FutureExt;
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    let (mut quit_tx, quit_rx) = channel(0);
    let (mut c, finished) = boring("Joe", quit_rx);

    task::block_on(async {
        // The loop will iterate printing Joe's messages until the loop finishes.
        for _ in 0i32..(thread_rng().gen_range(1, 10)) {
            println!("{}", c.next().await.unwrap());
        }

        println!("main: Quit please ...");
        quit_tx.send(()).await.expect("sending quit");
        finished.wait(); // Note: cannot use await keyword
        println!("main: Good. I'm out of here.");
    });
}

fn boring(message: &str, mut quit_rx: Receiver<()>) -> (Receiver<String>, WaitGroup) {
    let message_for_closure = message.to_owned();
    let (mut tx, rx) = channel(0);

    let wg = WaitGroup::new();
    let wg_for_closure = wg.clone();

    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            select! {
                _ = tx.send(msg).fuse() => { /* do nothing */ },
                _ = quit_rx.next() => {
                    println!("{}: Oh ok, I'll quit", message_for_closure);
                    drop(wg_for_closure);
                    return
                },
            }
            helpers::sleep(thread_rng().gen_range(0, 1000));
        }
    });

    (rx, wg)
}
