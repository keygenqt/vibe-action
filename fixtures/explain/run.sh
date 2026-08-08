#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- explain -f './fixtures/explain/arkts_sample.ets'
cargo run -- explain -f './fixtures/explain/bash_sample.sh'
cargo run -- explain -f './fixtures/explain/batch_sample.bat'
cargo run -- explain -f './fixtures/explain/csharp_sample.cs'
cargo run -- explain -f './fixtures/explain/dart_sample.dart'
cargo run -- explain -f './fixtures/explain/go_sample.go'
cargo run -- explain -f './fixtures/explain/java_sample.java'
cargo run -- explain -f './fixtures/explain/js_sample.js'
cargo run -- explain -f './fixtures/explain/kotlin_sample.kt'
cargo run -- explain -f './fixtures/explain/python_sample.py'
cargo run -- explain -f './fixtures/explain/rust_sample.rs'
cargo run -- explain -f './fixtures/explain/swift_sample.swift'
cargo run -- explain -f './fixtures/explain/ts_sample.ts'
