#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- review './fixtures/review/rust_unsafe.rs'
cargo run -- review './fixtures/review/python_memory.py'
cargo run -- review './fixtures/review/java_concurrency.java'
