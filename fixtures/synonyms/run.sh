#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- synonyms 'function'
cargo run -- synonyms 'variable'
cargo run -- synonyms 'iterate'
cargo run -- synonyms 'callback'
