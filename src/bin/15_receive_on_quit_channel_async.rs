// Note: async_std's channel is marked unstable. This example doesn't currently work.
use async_std::sync::{channel, Receiver, Sender};
use async_std::task;
use futures::future::FutureExt;
use futures::select;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    task::block_on(async {
        let quit_channel = channel(1);
        // Using `ref` to prevent move out of `quit_channel` before we pass to `boring`
        let (ref quit_tx, ref quit_rx) = quit_channel;

        let c = boring("Joe", quit_channel.clone());

        // The loop will iterate printing Joe's messages until the overall timeout occurs.
        for _ in 0..(thread_rng().gen_range(1, 10)) {
            println!("{}", c.recv().await.expect("receiving from joe"));
        }

        quit_tx.send("Bye!".to_owned()).await;

        // TODO this is flakey
        // Required to give Joe time to pick up our message, or we'll end up reading it here.
        task::sleep(std::time::Duration::from_secs(2)).await;

        println!("Joe said: {}", quit_rx.recv().await.expect("final receive"));
    });
}

fn boring(message: &str, quit_channel: (Sender<String>, Receiver<String>)) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (tx, rx) = channel(10);

    task::spawn(async move {
        let (quit_tx, quit_rx) = quit_channel;

        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            select! {
                _ = tx.send(msg).fuse() => { /* nothing to do */ },
                quit_msg = quit_rx.recv().fuse() => {
                    println!("main said: {}", quit_msg.unwrap());
                    quit_tx.send("See you!".to_owned()).await;
                },
            }
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    rx
}
