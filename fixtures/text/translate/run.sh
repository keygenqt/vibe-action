#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

# English -> Russian
cargo run -- text translate './fixtures/text/translate/english_sample.md' -l Russian
# English -> Chinese
cargo run -- text translate './fixtures/text/translate/english_sample.md' -l Chinese
# Russian -> English
cargo run -- text translate './fixtures/text/translate/russian_sample.md' -l English
# Russian -> Chinese
cargo run -- text translate './fixtures/text/translate/russian_sample.md' -l Chinese
# Chinese -> English
cargo run -- text translate './fixtures/text/translate/chinese_sample.md' -l English
# Chinese -> Russian
cargo run -- text translate './fixtures/text/translate/chinese_sample.md' -l Russian
