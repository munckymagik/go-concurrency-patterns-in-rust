use async_std::task;
use futures::channel::mpsc::{channel, Receiver};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    let mut c = fan_in(boring("Joe"), boring("Ann"));

    task::block_on(async {
        for _ in 0i32..10 {
            match c.next().await {
                Some(msg) => println!("{}", msg),
                None => break,
            };
        }
    });

    println!("You're both boring; I'm leaving.");
}

fn fan_in<T: 'static + Send>(mut input1: Receiver<T>, mut input2: Receiver<T>) -> Receiver<T> {
    let (mut tx, rx) = channel(0);
    let mut tx2 = tx.clone();

    task::spawn(async move {
        loop {
            let msg = input1.next().await.expect("input1 recv failed");
            tx.send(msg).await.expect("input1 send failed");
        }
    });
    task::spawn(async move {
        loop {
            let msg = input2.next().await.expect("input2 recv failed");
            tx2.send(msg).await.expect("input2 send failed");
        }
    });

    rx
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
