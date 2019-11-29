//! Based on Go example
//! [slide 42: "Example: Google Search"](https://talks.golang.org/2012/concurrency.slide#42)
//!
//! Q: What does Google search do?
//!
//! A: Given a query, return a page of search results (and some ads).
//!
//! Q: How do we get the search results?
//!
//! A: Send the query to Web search, Image search, YouTube, Maps, News,etc., then mix the results.
//!
//! How do we implement this?
//!
//! We can simulate the search function, much as we simulated conversation before.
//!
//! ## Google Search 1.0
//!
//! The `google` function takes a query and returns a `Vec` of results (which are just `Strings`).
//!
//! `google` invokes WEB, IMAGE, and VIDEO searches serially, appending them to the results.
//!
use std::{thread, time};

mod helpers;

struct FakeSearch<'a> {
    kind: &'a str,
}

impl<'a> FakeSearch<'a> {
    const fn new(kind: &'a str) -> Self {
        Self { kind }
    }

    fn call(&self, query: &str) -> String {
        thread::sleep(helpers::rand_duration(0, 100));
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
