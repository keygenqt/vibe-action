#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- comment -f './fixtures/comment/ts_sample.ts'
cargo run -- comment -f './fixtures/comment/java_sample.java'
cargo run -- comment -f './fixtures/comment/go_sample.go'
cargo run -- comment -f './fixtures/comment/csharp_sample.cs'
cargo run -- comment -f './fixtures/comment/swift_sample.swift'
cargo run -- comment -f './fixtures/comment/dart_sample.dart'
cargo run -- comment -f './fixtures/comment/bash_sample.sh'
cargo run -- comment -f './fixtures/comment/batch_sample.bat'
cargo run -- comment -f './fixtures/comment/arkts_sample.ets'
cargo run -- comment -f './fixtures/comment/js_sample.js'
cargo run -- comment -f './fixtures/comment/kotlin_sample.kt'
cargo run -- comment -f './fixtures/comment/python_sample.py'
cargo run -- comment -f './fixtures/comment/rust_sample.rs'
