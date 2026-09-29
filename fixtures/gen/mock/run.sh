#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- gen mock '5 users with id, name, email, and active status'
cargo run -- gen mock '3 products with sku, name, price, and stock quantity' -f yaml
cargo run -- gen mock '4 books with isbn, title, author, and publication year' -f csv
