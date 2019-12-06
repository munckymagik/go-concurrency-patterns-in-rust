//! Based on Go example
//! [slide 35: "Timeout using select"](https://talks.golang.org/2012/concurrency.slide#35)
//!
//! The `timeout_after` function returns a
//! [futures_timer::Delay](https://docs.rs/futures-timer/2.0.2/futures_timer/struct.Delay.html),
//! which is a [future](https://doc.rust-lang.org/std/future/trait.Future.html)
//! that resolves only after the specified duration has elapsed.
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
