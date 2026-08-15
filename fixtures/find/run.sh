#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. Find authentication logic
cargo run -- find 'authentication or login logic' -p './fixtures/find/mock_project'

# 2. Find database connection
cargo run -- find 'database connection or query' -p './fixtures/find/mock_project'

# 3. Find formatting helpers
cargo run -- find 'utility functions for formatting' -p './fixtures/find/mock_project'
