#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- vision describe './fixtures/vision/describe/screenshot_1.png'
cargo run -- vision describe './fixtures/vision/describe/screenshot_2.png'
cargo run -- vision describe 'https://vibe-action.keygenqt.com/assets/images/favicon.png'
