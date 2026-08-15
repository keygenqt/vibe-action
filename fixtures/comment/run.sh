#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- comment './fixtures/comment/arkts_sample.ets'
cargo run -- comment './fixtures/comment/bash_sample.sh'
cargo run -- comment './fixtures/comment/batch_sample.bat'
cargo run -- comment './fixtures/comment/csharp_sample.cs'
cargo run -- comment './fixtures/comment/dart_sample.dart'
cargo run -- comment './fixtures/comment/go_sample.go'
cargo run -- comment './fixtures/comment/java_sample.java'
cargo run -- comment './fixtures/comment/js_sample.js'
cargo run -- comment './fixtures/comment/kotlin_sample.kt'
cargo run -- comment './fixtures/comment/python_sample.py'
cargo run -- comment './fixtures/comment/rust_sample.rs'
cargo run -- comment './fixtures/comment/swift_sample.swift'
cargo run -- comment './fixtures/comment/ts_sample.ts'
