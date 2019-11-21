use rand::{thread_rng, Rng};
use std::time;

mod helpers;

struct FakeSearch<'a> {
    kind: &'a str,
}

impl<'a> FakeSearch<'a> {
    const fn new(kind: &'a str) -> Self {
        Self { kind }
    }

    fn call(&self, query: &str) -> String {
        helpers::sleep(thread_rng().gen_range(0, 100));
        format!("{} result for {}", self.kind, query)
    }
}

static WEB: FakeSearch = FakeSearch::new("web");
static IMAGE: FakeSearch = FakeSearch::new("image");
static VIDEO: FakeSearch = FakeSearch::new("video");

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
