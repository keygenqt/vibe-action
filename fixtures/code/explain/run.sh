#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- code explain './fixtures/code/explain/arkts_sample.ets'
cargo run -- code explain './fixtures/code/explain/bash_sample.sh'
cargo run -- code explain './fixtures/code/explain/batch_sample.bat'
cargo run -- code explain './fixtures/code/explain/csharp_sample.cs'
cargo run -- code explain './fixtures/code/explain/dart_sample.dart'
cargo run -- code explain './fixtures/code/explain/go_sample.go'
cargo run -- code explain './fixtures/code/explain/java_sample.java'
cargo run -- code explain './fixtures/code/explain/js_sample.js'
cargo run -- code explain './fixtures/code/explain/kotlin_sample.kt'
cargo run -- code explain './fixtures/code/explain/python_sample.py'
cargo run -- code explain './fixtures/code/explain/rust_sample.rs'
cargo run -- code explain './fixtures/code/explain/swift_sample.swift'
cargo run -- code explain './fixtures/code/explain/ts_sample.ts'
