use rand::{thread_rng, Rng};

mod helpers;

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        helpers::sleep(thread_rng().gen_range(0, 1000));
    }
}
