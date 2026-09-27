#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- text tone "$(cat './fixtures/text/tone/rude_english.txt')"
cargo run -- text tone "$(cat './fixtures/text/tone/rude_russian.txt')"
cargo run -- text tone "$(cat './fixtures/text/tone/rude_chinese.txt')"
