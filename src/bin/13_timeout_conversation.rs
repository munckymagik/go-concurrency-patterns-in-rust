use async_std::stream::{StreamExt, TimeoutError};
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;

use std::time;

mod helpers;

fn main() {
    task::block_on(async {
        let duration = time::Duration::from_millis(5000);
        let mut c = boring("Joe").timeout(duration);

        while let Some(item) = c.next().await {
            match item {
                Ok(s) => println!("{}", s),
                Err(TimeoutError { .. }) => {
                    println!("You talk too much.");
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
