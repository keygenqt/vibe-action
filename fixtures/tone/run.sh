#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. English
cargo run -- text tone "$(cat './fixtures/tone/rude_english.txt')"

# 2. Russian
cargo run -- text tone "$(cat './fixtures/tone/rude_russian.txt')"

# 3. Chinese
cargo run -- text tone "$(cat './fixtures/tone/rude_chinese.txt')"
