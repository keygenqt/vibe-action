#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- data find 'authentication or login logic' -p './fixtures/find/mock_project'
cargo run -- data find 'database connection or query' -p './fixtures/find/mock_project'
cargo run -- data find 'ничего искать не нужно' -p './fixtures/find/mock_project'
