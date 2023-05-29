serve:
  cargo run --bin server

dev-serve:
  cargo watch -c -q -x 'run --bin server RUST_BACKTRACE=1'


test:
  cargo watch -c -q -x "test --all --features integration-tests"
