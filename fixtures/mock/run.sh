#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. Users in JSON (default)
cargo run -- mock -q '5 users with id, name, email, and active status'

# 2. Products in YAML
cargo run -- mock -q '3 products with sku, name, price, and stock quantity' -f yaml

# 3. Books in CSV
cargo run -- mock -q '4 books with isbn, title, author, and publication year' -f csv
