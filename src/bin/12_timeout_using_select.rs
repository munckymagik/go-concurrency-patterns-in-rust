use async_std::future;
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::{FusedFuture, FutureExt};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};
use std::time;

mod helpers;

fn main() {
    let mut c = boring("Joe");

    task::block_on(async {
        loop {
            // In each loop, Joe has up to 500 ms to respond or the programme times-out.
            let mut timeout = timeout_after(500);

            select! {
                s = c.next() => println!("{}", s.unwrap()),
                _ = timeout => {
                    println!("You're too slow.");
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
            helpers::sleep(thread_rng().gen_range(0, 1000));
        }
    });

    rx
}
