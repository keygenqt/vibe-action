#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- code review './fixtures/review/rust_unsafe.rs'
cargo run -- code review './fixtures/review/python_memory.py'
cargo run -- code review './fixtures/review/java_concurrency.java'
