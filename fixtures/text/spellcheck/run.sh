#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- text spellcheck './fixtures/text/spellcheck/dirty_english.txt'
cargo run -- text spellcheck './fixtures/text/spellcheck/dirty_russian.txt'
cargo run -- text spellcheck './fixtures/text/spellcheck/dirty_markdown.md'
cargo run -- text spellcheck './fixtures/text/spellcheck/clean_english.txt'
