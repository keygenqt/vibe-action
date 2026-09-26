#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- code comment './fixtures/comment/arkts_sample.ets'
cargo run -- code comment './fixtures/comment/bash_sample.sh'
cargo run -- code comment './fixtures/comment/batch_sample.bat'
cargo run -- code comment './fixtures/comment/csharp_sample.cs'
cargo run -- code comment './fixtures/comment/dart_sample.dart'
cargo run -- code comment './fixtures/comment/go_sample.go'
cargo run -- code comment './fixtures/comment/java_sample.java'
cargo run -- code comment './fixtures/comment/js_sample.js'
cargo run -- code comment './fixtures/comment/kotlin_sample.kt'
cargo run -- code comment './fixtures/comment/python_sample.py'
cargo run -- code comment './fixtures/comment/rust_sample.rs'
cargo run -- code comment './fixtures/comment/swift_sample.swift'
cargo run -- code comment './fixtures/comment/ts_sample.ts'
