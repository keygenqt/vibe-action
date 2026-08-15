#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# English -> Russian
cargo run -- translate-large './fixtures/translate-large/english_sample.md' -l Russian

# English -> Chinese
cargo run -- translate-large './fixtures/translate-large/english_sample.md' -l Chinese

# Russian -> English
cargo run -- translate-large './fixtures/translate-large/russian_sample.md' -l English

# Russian -> Chinese
cargo run -- translate-large './fixtures/translate-large/russian_sample.md' -l Chinese

# Chinese -> English
cargo run -- translate-large './fixtures/translate-large/chinese_sample.md' -l English

# Chinese -> Russian
cargo run -- translate-large './fixtures/translate-large/chinese_sample.md' -l Russian
