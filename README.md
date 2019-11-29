# Go concurrency patterns in Rust

An attempt to implement Rob Pike's examples from [Go Concurrency Patterns](https://www.youtube.com/watch?v=f6kdp27TYZs), but in Rust rather than
Go.

## To run the examples:

Look at the names of the files in `src/bin` these will be the names of the
binaries to run. Then use `cargo run --bin <name>` to run them. E.g.

```bash
cargo run --bin 01_boring
cargo run --bin 02_boring_rand_sleep
# ... and so on.
```

## To generate Rust docs for the examples

```bash
cargo doc --bins --no-deps
open target/doc/01_boring/index.html
```
