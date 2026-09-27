#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- vision whois './fixtures/vision/whois/photo_1.jpg'
cargo run -- vision whois './fixtures/vision/whois/photo_2.jpg'
