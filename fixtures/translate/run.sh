#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# English -> Russian
cargo run -- text translate './fixtures/translate/english_sample.md' -l Russian

# English -> Chinese
cargo run -- text translate './fixtures/translate/english_sample.md' -l Chinese

# Russian -> English
cargo run -- text translate './fixtures/translate/russian_sample.md' -l English

# Russian -> Chinese
cargo run -- text translate './fixtures/translate/russian_sample.md' -l Chinese

# Chinese -> English
cargo run -- text translate './fixtures/translate/chinese_sample.md' -l English

# Chinese -> Russian
cargo run -- text translate './fixtures/translate/chinese_sample.md' -l Russian
