

use std::thread;
use futures::Future;
use futures::sync::oneshot::{self,Sender,Receiver};

// The limit for this example seems to be the maximum number of threads per process, which on
// my Mac appears to be 2048 and is confirmed by the output of `sysctl kern.num_taskthreads`
const MAX_THREADS: usize = 2048;

fn f(left: Sender<i64>, right: Receiver<i64>) {
    let val = right.wait().expect("wait problem");
    left.send(val + 1).expect("receiver hung up already");
}

fn main() {
    let n = MAX_THREADS - 2;
    let (mut rightmost_sender, leftmost_receiver) = oneshot::channel::<i64>();

    for _ in 0..n {
        let (next_sender, this_receiver) = oneshot::channel::<i64>();
        thread::spawn(move || f(rightmost_sender, this_receiver));
        rightmost_sender = next_sender;
    }

    thread::spawn(move || rightmost_sender.send(1).expect("1st receiver hung up already"));
    println!("{}", leftmost_receiver.wait().unwrap());
}
