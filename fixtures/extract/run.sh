#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- extract -f './fixtures/extract/server.log' -q 'extract all ERROR lines'
cargo run -- extract -f './fixtures/extract/transactions.csv' -q 'extract all failed transactions'
cargo run -- extract -f './fixtures/extract/config.env' -q 'extract all database connection settings'
cargo run -- extract -f './fixtures/extract/crash_report.txt' -q 'find lines containing NullReferenceException'
cargo run -- extract -f './fixtures/extract/server.log' -q 'find all CRITICAL errors'
