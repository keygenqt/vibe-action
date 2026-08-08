#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# English -> Russian
cargo run -- translate-small -f './fixtures/translate-small/english_sample.md' -l Russian

# English -> Chinese
cargo run -- translate-small -f './fixtures/translate-small/english_sample.md' -l Chinese

# Russian -> English
cargo run -- translate-small -f './fixtures/translate-small/russian_sample.md' -l English

# Russian -> Chinese
cargo run -- translate-small -f './fixtures/translate-small/russian_sample.md' -l Chinese

# Chinese -> English
cargo run -- translate-small -f './fixtures/translate-small/chinese_sample.md' -l English

# Chinese -> Russian
cargo run -- translate-small -f './fixtures/translate-small/chinese_sample.md' -l Russian
