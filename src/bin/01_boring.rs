mod helpers;

fn main() {
    boring();
}

fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        helpers::sleep(500);
    }
}
