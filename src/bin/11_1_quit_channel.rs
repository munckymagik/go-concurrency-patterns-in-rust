//! Based on Go example
//! [slide 37: "Quit channel"](https://talks.golang.org/2012/concurrency.slide#37)
//!
//! Instead of timing out, we can tell Joe to stop when we're tired of listening to him.
//!
use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::future::FutureExt;
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let (mut quit_sender, quit_receiver) = channel(0);
    let mut c = boring("Joe", quit_receiver);

    for _ in 0i32..(thread_rng().gen_range(1, 10)) {
        println!("{}", c.next().await.unwrap());
    }

    println!("main: telling Joe to quit ...");
    quit_sender.send(()).await.expect("sending quit");

    println!("main: bye!");
}

fn boring(message: &str, mut quit_receiver: Receiver<()>) -> Receiver<String> {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            select! {
                _ = sender.send(msg).fuse() => { /* do nothing */ },
                _ = quit_receiver.next() => {
                    println!("{}: pretending to clear up ... ok bye!", message_for_closure);
                    return
                },
            }
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }
    });

    receiver
}
