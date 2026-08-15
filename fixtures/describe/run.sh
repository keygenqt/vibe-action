#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- describe './fixtures/describe/screenshot_1.png'
cargo run -- describe './fixtures/describe/screenshot_2.png'
cargo run -- describe 'https://vibe-action.keygenqt.com/assets/images/favicon.png'
