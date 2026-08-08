#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. Find authentication logic
cargo run -- find -p './fixtures/find/mock_project' -q 'authentication or login logic'

# 2. Find database connection
cargo run -- find -p './fixtures/find/mock_project' -q 'database connection or query'

# 3. Find formatting helpers
cargo run -- find -p './fixtures/find/mock_project' -q 'utility functions for formatting'
