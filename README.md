# Go concurrency patterns in Rust

## What?

Rob Pike's examples from
[Go Concurrency Patterns](https://www.youtube.com/watch?v=f6kdp27TYZs), but in
Rust.

* [Rob's talk](https://www.youtube.com/watch?v=f6kdp27TYZs)
* [Rob's slides](https://talks.golang.org/2012/concurrency.slide)

## How?

All examples use the [async-std](https://crates.io/crates/async-std)
and [futures](https://crates.io/crates/futures) crates, except where standard threads are used to demonstrate contrasting behaviour.

## Why?

Initially this was a learning exercise for me. Back when I first started
learning Go, Rob's talk really helped me get a good intuition for go-routines
and channels. I wanted to compare Rust's approach to asynchrony by relating
back to what I already understood from Go.

The complexity of the examples increases at a comfortable pace. Each builds on
the knowledge gained in those that came before.

This was a really good practical way to learn Rust's async features and
libraries, without the burden of needing to build an actual application at the
same time.

I highly recommend this as an exercise for others.

## Running and browsing the examples

Each example is documented using [rustdoc](https://doc.rust-lang.org/rustdoc/)
comments, with text from Rob's original slides adapted for Rust.

Instructions for running and browsing follow:

### 01_boring

* Run: `cargo run --bin 01_boring`
* [View source](src/bin/01_boring.rs)

### 02_boring_rand_sleep

* Run: `cargo run --bin 02_boring_rand_sleep`
* [View source](src/bin/02_boring_rand_sleep.rs)

### 03_ignore

* Run: `cargo run --bin 03_ignore`
* [View source](src/bin/03_ignore.rs)

### 04_ignore_less

* Run: `cargo run --bin 04_ignore_less`
* [View source](src/bin/04_ignore_less.rs)

### 05_channel

* Run: `cargo run --bin 05_channel`
* [View source](src/bin/05_channel.rs)

### 06_generator_function

* Run: `cargo run --bin 06_generator_function`
* [View source](src/bin/06_generator_function.rs)

### 07_channels_as_handle_on_svc

* Run: `cargo run --bin 07_channels_as_handle_on_svc`
* [View source](src/bin/07_channels_as_handle_on_svc.rs)

### 08_1_fan_in_multiplexing

* Run: `cargo run --bin 08_1_fan_in_multiplexing`
* [View source](src/bin/08_1_fan_in_multiplexing.rs)

### 08_2_fan_in_restoring_sequence

* Run: `cargo run --bin 08_2_fan_in_restoring_sequence`
* [View source](src/bin/08_2_fan_in_restoring_sequence.rs)

### 08_3_fan_in_with_select

* Run: `cargo run --bin 08_3_fan_in_with_select`
* [View source](src/bin/08_3_fan_in_with_select.rs)

### 09_1_timeout_select

* Run: `cargo run --bin 09_1_timeout_select`
* [View source](src/bin/09_1_timeout_select.rs)

### 09_2_timeout_combinator

* Run: `cargo run --bin 09_2_timeout_combinator`
* [View source](src/bin/09_2_timeout_combinator.rs)

### 10_1_timeout_conversation_select

* Run: `cargo run --bin 10_1_timeout_conversation_select`
* [View source](src/bin/10_1_timeout_conversation_select.rs)

### 10_2_timeout_conversation_combinator

* Run: `cargo run --bin 10_2_timeout_conversation_combinator`
* [View source](src/bin/10_2_timeout_conversation_combinator.rs)

### 11_1_quit_channel

* Run: `cargo run --bin 11_1_quit_channel`
* [View source](src/bin/11_1_quit_channel.rs)

### 11_2_quit_channel_wait_for_done

* Run: `cargo run --bin 11_2_quit_channel_wait_for_done`
* [View source](src/bin/11_2_quit_channel_wait_for_done.rs)

### 12_daisy_chain_thread_limit

* Run: `cargo run --bin 12_daisy_chain_thread_limit`
* [View source](src/bin/12_daisy_chain_thread_limit.rs)

### 12_daisy_chain_unlimited

* Run: `cargo run --bin 12_daisy_chain_unlimited`
* [View source](src/bin/12_daisy_chain_unlimited.rs)

### 13_1_fake_google_search

* Run: `cargo run --bin 13_1_fake_google_search`
* [View source](src/bin/13_1_fake_google_search.rs)

### 13_2_fake_google_search_async

* Run: `cargo run --bin 13_2_fake_google_search_async`
* [View source](src/bin/13_2_fake_google_search_async.rs)

### 13_3_fake_google_search_timeout

* Run: `cargo run --bin 13_3_fake_google_search_timeout`
* [View source](src/bin/13_3_fake_google_search_timeout.rs)

### 13_4_fake_google_search_replication

TODO I'm still to complete this.

* [Rob's slide](https://talks.golang.org/2012/concurrency.slide#48)

## Browsing the docs for the examples as `rustdoc`

To generate `rustdoc` HTML documentation:

```bash
cargo doc --bins --no-deps
```

Then open [target/doc/01_boring/index.html](target/doc/01_boring/index.html)
in your browser.
