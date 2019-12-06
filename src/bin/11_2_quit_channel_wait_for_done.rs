//! Based on Go example
//! [slide 38: "Receive on quit channel"](https://talks.golang.org/2012/concurrency.slide#38)
//!
//! How do we know it's finished? Wait for it to tell us it's done. However, this is where we
//! diverge from the Go example. Because Rust async tasks are futures we can wait for the task to
//! exit by calling `await` on the
//! [JoinHandle](https://docs.rs/async-std/1.2.0/async_std/task/struct.JoinHandle.html)
//! returned by `task::spawn`.
//!
//! We can also obtain a final value from the `JoinHandle` too.
//!
//! Finally, in the previous example we sent an explicit message on the quit
//! channel. But we don't really need to send a message. When all senders have
//! been dropped the receiver will shut down naturally.
//!
use async_std::task;
use async_std::task::JoinHandle;
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
    let (quit_sender, quit_receiver) = channel(0);
    let (finished, mut c) = boring("Joe", quit_receiver);

    for _ in 0i32..(thread_rng().gen_range(1, 10)) {
        println!("{}", c.next().await.unwrap());
    }

    println!("main: telling Joe to quit ...");
    drop(quit_sender); // dropping the sender will signal to the receiver that
                       // communication on the channel has finished.

    println!("main: waiting for Joe to clear up ...");
    let final_words = finished.await;
    println!("main: Joe says '{}'", final_words);
}

fn boring(
    message: &str,
    mut quit_receiver: Receiver<()>,
) -> (JoinHandle<String>, Receiver<String>) {
    let message_for_closure = message.to_owned();
    let (mut sender, receiver) = channel(0);

    let handle = task::spawn(async move {
        for i in 0i32.. {
            let msg = format!("{} {}", message_for_closure, i);
            select! {
                _ = sender.send(msg).fuse() => { /* do nothing */ },

                // When main drops the sender, next() returns `None` to mark
                // the end of communication
                _ = quit_receiver.next() => {
                    break;
                },
            }
            task::sleep(helpers::rand_duration(0, 1000)).await;
        }

        println!("{}: pretending to clear up ...", message_for_closure);
        "See you!".to_owned() // This will be returned to main
    });

    (handle, receiver)
}
