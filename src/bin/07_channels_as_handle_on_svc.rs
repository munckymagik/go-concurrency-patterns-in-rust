use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    let mut joe = boring("Joe");
    let mut ann = boring("Ann");

    task::block_on(async {
        for _ in 0i32..5 {
            println!("{}", joe.next().await.expect("Receiving joe failed"));
            println!("{}", ann.next().await.expect("Receiving ann failed"));
        }
    });

    println!("You're both boring; I'm leaving.");
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
