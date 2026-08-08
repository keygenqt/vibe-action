#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- synonyms -q 'function'
cargo run -- synonyms -q 'variable'
cargo run -- synonyms -q 'iterate'
cargo run -- synonyms -q 'callback'
