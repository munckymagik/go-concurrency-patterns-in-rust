use async_std::task;
use futures::channel::mpsc::channel;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
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

async fn google(query: &str) -> Vec<String> {
    let mut results = Vec::new();
    let (sender, mut receiver) = channel(0);
    let searches: [&FakeSearch; 3] = [&WEB, &IMAGE, &VIDEO];

    for search in &searches {
        let search = search.to_owned();
        let query = query.to_owned();
        let mut sender = sender.to_owned();

        task::spawn(async move {
            let result = search.call(&query);
            sender.send(result).await.unwrap();
        });
    }

    // Drop the original sender so channel will close when all search tasks
    // have completed
    drop(sender);

    // Aggregate until the channel closes
    while let Some(result) = receiver.next().await {
        results.push(result);
    }

    results
}

fn main() {
    task::block_on(async {
        let start = time::Instant::now();
        let results = google("rust lang").await;
        let elapsed = start.elapsed();

        println!("Result: {:#?}", results);
        println!("Elapsed: {}ms", helpers::to_millis(elapsed));
    })
}
