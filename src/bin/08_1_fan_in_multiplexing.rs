//! Based on Go example
//! [slide 27: "Multiplexing"](https://talks.golang.org/2012/concurrency.slide#27)
//!
//! The previous programs make Joe and Ann count in lockstep.
//!
//! We can instead use a fan-in function to let whosoever is ready talk.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let mut c = fan_in(boring("Joe"), boring("Ann"));

    for _ in 0i32..10 {
        match c.next().await {
            Some(msg) => println!("{}", msg),
            None => break,
        };
    }

    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(mut input1: Receiver<T>, mut input2: Receiver<T>) -> Receiver<T> {
    let (mut sender, receiver) = channel(0);
    let mut sender2 = sender.clone();

    task::spawn(async move {
        while let Some(msg) = input1.next().await {
            sender.send(msg).await.expect("input1 send failed");
        }
    });
    task::spawn(async move {
        while let Some(msg) = input2.next().await {
            sender2.send(msg).await.expect("input2 send failed");
        }
    });

    receiver
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
