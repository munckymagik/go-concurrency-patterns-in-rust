use async_std::task;

mod helpers;

fn main() {
    task::spawn(boring());
    println!("I got bored");
}

async fn boring() {
    for i in 0.. {
        println!("boring! {}", i);
        task::sleep(helpers::rand_duration(0, 1000)).await;
    }
}
