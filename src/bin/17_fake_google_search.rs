#[macro_use]
extern crate lazy_static;

use rand::{thread_rng, Rng};
use std::time;

mod helpers;

struct FakeSearch {
    kind: String,
}

impl FakeSearch {
    fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_owned(),
        }
    }

    fn call(&self, query: &str) -> String {
        helpers::sleep(thread_rng().gen_range(0, 100));
        format!("{} result for {}", self.kind, query)
    }
}

lazy_static! {
    static ref WEB: FakeSearch = FakeSearch::new("web");
    static ref IMAGE: FakeSearch = FakeSearch::new("image");
    static ref VIDEO: FakeSearch = FakeSearch::new("video");
}

fn google(query: &str) -> Vec<String> {
    let mut results = Vec::new();
    results.push(WEB.call(query));
    results.push(IMAGE.call(query));
    results.push(VIDEO.call(query));
    results
}

fn main() {
    let start = time::Instant::now();
    let results = google("rust lang");
    let elapsed = start.elapsed();

    println!("Result: {:#?}", results);
    println!("Elapsed: {}ms", helpers::to_millis(elapsed));
}
