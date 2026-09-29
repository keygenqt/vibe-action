#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- gen regex 'match an email address' -e 'user@example.com'
cargo run -- gen regex 'extract domain name from a URL' -e 'https://www.example.com/path'
cargo run -- gen regex 'match a valid IPv4 address' -e '192.168.1.1'
cargo run -- gen regex 'match a date in YYYY-MM-DD format'
cargo run -- gen regex 'match a US phone number' -e '(123) 456-7890'
