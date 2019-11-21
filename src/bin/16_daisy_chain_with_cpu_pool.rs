use futures;
use futures_cpupool;

use futures::{BoxFuture, Future};
use futures::future::join_all;
use futures::sync::oneshot::{self,Sender,Receiver,Canceled};

fn f(left: Sender<i64>, right: Receiver<i64>) -> BoxFuture<(), futures::Canceled> {
    right.and_then(move |val| {
        left.send(val + 1).map_err(|_| Canceled)
    }).boxed()
}

fn main() {
    let pool = futures_cpupool::Builder::new().create();
    let n = 10_000;
    let (mut rightmost_sender, leftmost_receiver) = oneshot::channel::<i64>();
    let mut futures = Vec::with_capacity(n);

    for _ in 0..n {
        let (next_sender, this_receiver) = oneshot::channel::<i64>();
        let future = pool.spawn_fn(move || f(rightmost_sender, this_receiver));
        futures.push(future);
        rightmost_sender = next_sender;
    }

    let start = pool.spawn_fn(move || {
        rightmost_sender.send(1).map_err(|_| Canceled)
    });

    let last = pool.spawn_fn(move || {
        println!("Waiting to receive result ...");
        leftmost_receiver.map(|result| {
            println!("RESULT: {}", result);
        })
    });

    futures.push(start);
    futures.push(last);

    join_all(futures)
        .wait()
        .expect("waiting for futures to complete");
}
