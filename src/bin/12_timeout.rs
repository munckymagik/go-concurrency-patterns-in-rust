use async_std::future::{timeout, TimeoutError};
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use std::time;

mod helpers;

fn main() {
    let mut c = boring("Joe");
    let duration = time::Duration::from_millis(500);

    task::block_on(async {
        loop {
            // In each loop, Joe has up to 500 ms to respond or the program times-out.
            match timeout(duration, c.next()).await {
                Ok(s) => println!("{}", s.unwrap()),
                Err(TimeoutError { .. }) => {
                    println!("You're too slow.");
                    return;
                }
            }
        }
    });
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
