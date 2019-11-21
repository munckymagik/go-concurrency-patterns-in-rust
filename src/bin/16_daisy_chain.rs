use async_std::task;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

async fn f(mut left: Sender<i64>, mut right: Receiver<i64>) {
    let val = right.next().await.expect("receiver problem");
    left.send(val + 1).await.expect("receiver hung up already");
}

fn main() {
    let n = 10_000; // Note: not limited by OS max threads limit

    task::block_on(async {
        let (mut rightmost_sender, mut leftmost_receiver) = channel(0);

        for _ in 0..(n - 1) {
            let (next_sender, this_receiver) = channel(0);
            task::spawn(f(rightmost_sender, this_receiver));
            rightmost_sender = next_sender;
        }

        task::spawn(async move { rightmost_sender.send(1).await });

        println!(
            "{}",
            leftmost_receiver
                .next()
                .await
                .expect("receiving final value")
        );
    });
}
