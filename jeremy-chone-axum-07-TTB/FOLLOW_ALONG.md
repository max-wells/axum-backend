
YT: https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q





```bash
cargo install cargo-watch
cargo update
```


# Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - For test.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

# Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For test.
cargo test quick_dev -- --nocapture
```