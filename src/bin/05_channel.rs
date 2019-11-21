use async_std::task;
use futures::channel::mpsc::{channel, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    let (tx, mut rx) = channel(0);
    task::spawn(boring("boring!", tx));

    task::block_on(async {
        for _ in 0i32..5 {
            println!("You say: {}", rx.next().await.expect("Receiving failed"));
        }
    });

    println!("You're boring; I'm leaving.");
}

async fn boring(msg: &str, mut tx: Sender<String>) {
    for i in 0i32.. {
        let msg_i = format!("{} {}", msg, i);
        tx.send(msg_i)
            .await
            .expect("Failed to send message to channel");
        helpers::sleep(thread_rng().gen_range(0, 1000));
    }
}
