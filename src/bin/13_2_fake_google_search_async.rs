//! Based on Go example
//! [slide 46: "Google Search 2.0"](https://talks.golang.org/2012/concurrency.slide#46)
//!
//! Run the Web, Image, and Video searches concurrently, and wait for all results.
//!
//! No locks.  No condition variables.  No callbacks.
//!
use async_std::task;
use futures::channel::mpsc::channel;
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

    // Drop the original sender so channel will close when all search tasks
    // have completed
    drop(sender);

    // Aggregate results until the channel closes
    while let Some(result) = receiver.next().await {
        results.push(result);
    }

    results
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
