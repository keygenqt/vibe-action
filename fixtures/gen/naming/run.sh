#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- gen naming 'function to sort actions by dependency for Rust'
cargo run -- gen naming 'a class that manages a pool of database connections'
cargo run -- gen naming 'a constant for the maximum retry attempts'
cargo run -- gen naming 'a variable holding the current user session token'
cargo run -- gen naming 'a boolean flag to check if the cache is enabled for Kotlin'
