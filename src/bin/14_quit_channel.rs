use async_std::task;
use async_std::task::JoinHandle;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::FutureExt;
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    let (mut quit_tx, quit_rx) = channel(0);
    let (handle, mut c) = boring("Joe", quit_rx);

    task::block_on(async {
        // The loop will iterate printing Joe's messages until the loop finishes.
        for _ in 0i32..(thread_rng().gen_range(1, 10)) {
            println!("{}", c.next().await.unwrap());
        }

        println!("quitting ...");
        quit_tx.send(()).await.expect("sending quit");

        println!("waiting ...");
        handle.await;
    });
}

fn boring(message: &str, mut quit_rx: Receiver<()>) -> (JoinHandle<()>, Receiver<String>) {
    let message_for_closure = message.to_owned();
    let (mut tx, rx) = channel(0);

    let handle = task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            select! {
                _ = tx.send(msg).fuse() => { /* do nothing */ },
                _ = quit_rx.next() => {
                    println!("Ok bye!");
                    return
                },
            }
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    (handle, rx)
}
