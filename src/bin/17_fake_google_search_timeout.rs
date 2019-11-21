#[macro_use]
extern crate chan;
#[macro_use]
extern crate lazy_static;


use std::{thread, time};
use rand::{thread_rng, Rng};

struct FakeSearch {
    kind: String
}

impl FakeSearch {
    fn new(kind: &str) -> Self {
        Self { kind: kind.to_owned() }
    }

    fn call(&self, query: &str) -> String {
        util::sleep(thread_rng().gen_range(0, 100));
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
    let (sender, receiver) = chan::r#async();
    let searches: [&FakeSearch; 3] = [&WEB, &IMAGE, &VIDEO];

    for search in &searches {
        let search = search.to_owned();
        let query = query.to_owned();
        let sender = sender.to_owned();
        thread::spawn(move || sender.send(search.call(&query)));
    }

    let timeout = chan::after(time::Duration::from_millis(80));
    for _ in 0..searches.len() {
        chan_select! {
            receiver.recv() -> s => results.push(s.unwrap()),
            timeout.recv() => {
                println!("timed out");
                return results;
            },
        }
    }

    results
}

mod util {
    use super::*;

    pub fn sleep(dur_ms: u64) {
        thread::sleep(time::Duration::from_millis(dur_ms));
    }

    pub fn to_millis(duration: time::Duration) -> f64 {
        let sec_ms = duration.as_secs() as f64 * 1e3;
        let subsec_ms = duration.subsec_nanos() as f64 / 1e6;
        sec_ms + subsec_ms
    }
}

fn main() {
    let start = time::Instant::now();
    let results = google("rust lang");
    let elapsed = start.elapsed();

    println!("Result: {:#?}", results);
    println!("Elapsed: {}ms", util::to_millis(elapsed));
}
