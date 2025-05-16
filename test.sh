#!/bin/bash

# Common logic
set -e
set -x
repomix

# Rust-specific logic
export RUST_BACKTRACE=1
# export RUST_BACKTRACE=full

# cargo clean
cargo fmt
cargo clippy
cargo test
# cargo bench
cargo build
cargo run --release -- --version --help
tree -h -L 2

echo "Running tests with coverage for Fractal Amadeus Rust implementation..."
cargo tarpaulin --verbose --workspace --out Html --output-dir coverage --fail-immediately --fail-under 100
echo "Test coverage report generated in fractal_amadeus_rs/coverage/"

echo
echo "All the tests stucceed successfully!"
echo
