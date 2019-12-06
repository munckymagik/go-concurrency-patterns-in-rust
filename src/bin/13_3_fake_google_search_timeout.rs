//! Based on Go example
//! [slide 47: "Google Search 2.1"](https://talks.golang.org/2012/concurrency.slide#47)
//!
//! Don't wait for slow servers. No locks.  No condition variables.  No callbacks.
//!
//! In this version we hard-limit the search to 80ms and only return results
//! successfully collected in that time.
//!
use async_std::future;
use async_std::task;
use futures::channel::mpsc::channel;
use futures::future::{FusedFuture, FutureExt};
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::time;

mod helpers;

struct FakeSearch<'a> {
    kind: &'a str,
}

impl<'a> FakeSearch<'a> {
    const fn new(kind: &'a str) -> Self {
        Self { kind }
    }

    async fn call(&self, query: &str) -> String {
        task::sleep(helpers::rand_duration(0, 100)).await;
        format!("{} result for {}", self.kind, query)
    }
}

static WEB: FakeSearch = FakeSearch::new("web");
static IMAGE: FakeSearch = FakeSearch::new("image");
static VIDEO: FakeSearch = FakeSearch::new("video");

async fn google(query: &str) -> Vec<String> {
    let mut results = Vec::new();
    let (sender, mut receiver) = channel(0);
    let searches: [&FakeSearch; 3] = [&WEB, &IMAGE, &VIDEO];

    for search in &searches {
        // Clone values so they can be safely transferred across the threads
        let search = search.to_owned();
        let query = query.to_owned();
        let mut sender = sender.to_owned();

        task::spawn(async move {
            // Perform the search
            let result = search.call(&query).await;

            // Send the result back over the channel
            sender.send(result).await.unwrap();
        });
    }

    // The searches have 80ms to report back or we just return whatever results
    // we have (which could be none).
    let mut timeout = timeout_after(80);
    for _ in 0..searches.len() {
        select! {
            s = receiver.next() => results.push(s.unwrap()),
            _ = timeout => {
                println!("timed out");
                break;
            },
        }
    }

    results
}

fn timeout_after(ms: u64) -> impl FusedFuture {
    let duration = time::Duration::from_millis(ms);
    let never = future::pending::<()>();
    future::timeout(duration, never).boxed().fuse()
}

fn main() {
    task::block_on(async_main());
}

async fn async_main() {
    let start = time::Instant::now();
    let results = google("rust lang").await;
    let elapsed = start.elapsed();

    println!("Result: {:#?}", results);
    println!("Elapsed: {}ms", helpers::to_millis(elapsed));
}
