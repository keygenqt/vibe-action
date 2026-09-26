#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- code explain './fixtures/explain/arkts_sample.ets'
cargo run -- code explain './fixtures/explain/bash_sample.sh'
cargo run -- code explain './fixtures/explain/batch_sample.bat'
cargo run -- code explain './fixtures/explain/csharp_sample.cs'
cargo run -- code explain './fixtures/explain/dart_sample.dart'
cargo run -- code explain './fixtures/explain/go_sample.go'
cargo run -- code explain './fixtures/explain/java_sample.java'
cargo run -- code explain './fixtures/explain/js_sample.js'
cargo run -- code explain './fixtures/explain/kotlin_sample.kt'
cargo run -- code explain './fixtures/explain/python_sample.py'
cargo run -- code explain './fixtures/explain/rust_sample.rs'
cargo run -- code explain './fixtures/explain/swift_sample.swift'
cargo run -- code explain './fixtures/explain/ts_sample.ts'
