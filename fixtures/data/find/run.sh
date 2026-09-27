#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- data find 'authentication or login logic' -p './fixtures/data/find/mock_project'
cargo run -- data find 'database connection or query' -p './fixtures/data/find/mock_project'
cargo run -- data find 'ничего искать не нужно' -p './fixtures/data/find/mock_project'
