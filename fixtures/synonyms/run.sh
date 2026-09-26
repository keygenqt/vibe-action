#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- gen synonyms 'function'
cargo run -- gen synonyms 'variable'
cargo run -- gen synonyms 'iterate'
cargo run -- gen synonyms 'callback'
