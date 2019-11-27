use async_std::future;
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::{FusedFuture, FutureExt};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use std::time;

mod helpers;

fn main() {
    let mut c = boring("Joe");
    let mut timeout = timeout_after(5000);

    task::block_on(async {
        // The loop will iterate printing Joe's messages until the overall timeout occurs.
        loop {
            select! {
                s = c.next() => println!("{}", s.unwrap()),
                _ = timeout => {
                    println!("You talk too much.");
                    return;
                },
            }
        }
    });
}

fn timeout_after(ms: u64) -> impl FusedFuture {
    let duration = time::Duration::from_millis(ms);
    let never = future::pending::<()>();
    future::timeout(duration, never).boxed().fuse()
}

fn boring(message: &str) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (mut tx, rx) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            tx.send(msg)
                .await
                .expect("Failed to send message to channel");
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    rx
}
