//! Based on Go example
//! [slide 26: "Channels as a handle on a service"](https://talks.golang.org/2012/concurrency.slide#26)
//!
//! Our boring function returns a `Receiver` for a channel that lets us communicate with the boring service it provides.
//!
//! We can have more instances of the service.
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
    let mut joe = boring("Joe"); // first service
    let mut ann = boring("Ann"); // second service

    for _ in 0i32..5 {
        println!("{}", joe.next().await.expect("receiving joe failed"));
        println!("{}", ann.next().await.expect("receiving ann failed"));
    }

    println!("You're both boring; I'm leaving.");
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
