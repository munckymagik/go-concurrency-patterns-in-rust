extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures::future::join_all;
use futures::sync::oneshot::{self,Sender,Receiver};

fn f(i: usize, left: Sender<i64>, right: Receiver<i64>) -> Result<(), futures::Canceled> {
    println!("{} f one", i);
    right.map(|val| {
        println!("{} f two", i);
        let r = left.send(val + 1).expect("problem sending on");
        println!("{} f three", i);
        r
    }).wait()
}

fn main() {
    let pool = futures_cpupool::Builder::new().create();
    let n = 3; //10_000;
    let (mut rightmost_sender, leftmost_receiver) = oneshot::channel::<i64>();
    let mut futures = Vec::with_capacity(n);

    for i in 0..n {
        let (next_sender, this_receiver) = oneshot::channel::<i64>();
        let future = pool.spawn_fn(move || f(i, rightmost_sender, this_receiver));
        futures.push(future);
        rightmost_sender = next_sender;
    }

    let start = pool.spawn_fn(move || {
        println!("Sending 1st");
        rightmost_sender.send(1).map(|_| {
            println!("1st sent");
        }).expect("send failed");
        Ok(())
    });

    let last = pool.spawn_fn(move || {
        println!("Waiting to receive result ...");
        leftmost_receiver.map(|result| {
            println!("RESULT: {}", result);
            result
        }).wait()
    });

    start.and_then(|_| join_all(futures))
        .and_then(|_| last)
        .wait()
        .expect("waiting for futures to complete");
}