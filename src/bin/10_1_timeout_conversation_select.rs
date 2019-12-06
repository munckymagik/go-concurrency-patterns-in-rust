//! Based on Go example
//! [slide 36: "Timeout for whole conversation using select"](https://talks.golang.org/2012/concurrency.slide#36)
//!
//! Create the timer once, outside the loop, to time out the entire conversation.
//!
//! (In the previous program, we had a timeout for each message.)
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::{FusedFuture, FutureExt};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use futures_timer::Delay;
use std::time;

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let mut c = boring("Joe");
    let mut timeout = timeout_after(5000);

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
}

fn timeout_after(ms: u64) -> impl FusedFuture {
    let duration = time::Duration::from_millis(ms);
    Delay::new(duration).boxed().fuse()
}

fn boring(message: &str) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            sender.send(msg).await.expect("sending failed");
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    receiver
}
