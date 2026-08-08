#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- naming -q 'function to sort actions by dependency for Rust'
cargo run -- naming -q 'a class that manages a pool of database connections'
cargo run -- naming -q 'a constant for the maximum retry attempts'
cargo run -- naming -q 'a variable holding the current user session token'
cargo run -- naming -q 'a boolean flag to check if the cache is enabled for Kotlin'
