#[macro_use]
extern crate lazy_static;

use futures::future::join_all;
use futures::Future;
use futures_cpupool::{CpuFuture, CpuPool};
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

fn google(pool: &CpuPool, searches: &[&'static FakeSearch], query: &str) -> Vec<String> {
    let futures = searches
        .iter()
        .map(|search| {
            let search = search.to_owned();
            let query = query.to_owned();
            pool.spawn_fn(move || Ok(search.call(&query)))
        })
        .collect::<Vec<CpuFuture<String, ()>>>();

    join_all(futures).wait().unwrap()
}

fn main() {
    let pool = CpuPool::new_num_cpus();
    let searches: [&FakeSearch; 3] = [&WEB, &IMAGE, &VIDEO];

    let start = time::Instant::now();
    let results = google(&pool, &searches, "rust lang");
    let elapsed = start.elapsed();

    println!("Result: {:#?}", results);
    println!("Elapsed: {}ms", helpers::to_millis(elapsed));
}
