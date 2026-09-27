#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- code comment './fixtures/code/comment/arkts_sample.ets'
cargo run -- code comment './fixtures/code/comment/bash_sample.sh'
cargo run -- code comment './fixtures/code/comment/batch_sample.bat'
cargo run -- code comment './fixtures/code/comment/csharp_sample.cs'
cargo run -- code comment './fixtures/code/comment/dart_sample.dart'
cargo run -- code comment './fixtures/code/comment/go_sample.go'
cargo run -- code comment './fixtures/code/comment/java_sample.java'
cargo run -- code comment './fixtures/code/comment/js_sample.js'
cargo run -- code comment './fixtures/code/comment/kotlin_sample.kt'
cargo run -- code comment './fixtures/code/comment/python_sample.py'
cargo run -- code comment './fixtures/code/comment/rust_sample.rs'
cargo run -- code comment './fixtures/code/comment/swift_sample.swift'
cargo run -- code comment './fixtures/code/comment/ts_sample.ts'
