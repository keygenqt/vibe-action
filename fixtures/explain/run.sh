#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- explain './fixtures/explain/arkts_sample.ets'
cargo run -- explain './fixtures/explain/bash_sample.sh'
cargo run -- explain './fixtures/explain/batch_sample.bat'
cargo run -- explain './fixtures/explain/csharp_sample.cs'
cargo run -- explain './fixtures/explain/dart_sample.dart'
cargo run -- explain './fixtures/explain/go_sample.go'
cargo run -- explain './fixtures/explain/java_sample.java'
cargo run -- explain './fixtures/explain/js_sample.js'
cargo run -- explain './fixtures/explain/kotlin_sample.kt'
cargo run -- explain './fixtures/explain/python_sample.py'
cargo run -- explain './fixtures/explain/rust_sample.rs'
cargo run -- explain './fixtures/explain/swift_sample.swift'
cargo run -- explain './fixtures/explain/ts_sample.ts'
