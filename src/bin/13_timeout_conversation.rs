use async_std::future;
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use std::time;

mod helpers;

fn main() {
    let duration = time::Duration::from_millis(5000);

    let fut = future::timeout(duration, async {
        let mut c = boring("Joe");

        while let Some(s) = c.next().await {
            println!("{}", s);
        }
    });

    match task::block_on(fut) {
        Err(future::TimeoutError { .. }) => println!("You talk too much."),
        _ => (),
    };
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
