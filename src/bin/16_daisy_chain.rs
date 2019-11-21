use chan;

use std::thread;
use chan::{Sender, Receiver};

// The limit for this example seems to be the maximum number of threads per process, which on
// my Mac appears to be 2048 and is confirmed by the output of `sysctl kern.num_taskthreads`
const MAX_THREADS: usize = 2048;

fn f(left: Sender<i64>, right: Receiver<i64>) {
    left.send(right.recv().unwrap() + 1);
}

fn main() {
    let n = MAX_THREADS - 2;
    let (mut rightmost_sender, leftmost_receiver) = chan::sync::<i64>(0);

    for _ in 0..n {
        let (next_sender, this_receiver) = chan::sync::<i64>(0);
        thread::spawn(move || f(rightmost_sender, this_receiver));
        rightmost_sender = next_sender;
    }

    thread::spawn(move || rightmost_sender.send(1));
    println!("{}", leftmost_receiver.recv().unwrap());
}
