#!/bin/bash

# Common logic
set -e
set -x
repomix

# Rust-specific logic
export RUST_BACKTRACE=1

cargo clippy
cargo fmt
cargo test
cargo bench
cargo build
cargo run --release -- --version

# echo "Running tests with coverage for Fractal Amadeus Rust implementation..."
# cargo tarpaulin --verbose --workspace --out Html --output-dir coverage
# echo "Test coverage report generated in fractal_amadeus_rs/coverage/"

