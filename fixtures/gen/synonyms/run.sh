#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- gen synonyms 'function'
cargo run -- gen synonyms 'variable'
cargo run -- gen synonyms 'iterate'
cargo run -- gen synonyms 'callback'
