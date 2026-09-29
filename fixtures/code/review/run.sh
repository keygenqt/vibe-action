#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- code review './fixtures/code/review/rust_unsafe.rs'
cargo run -- code review './fixtures/code/review/python_memory.py'
cargo run -- code review './fixtures/code/review/java_concurrency.java'
