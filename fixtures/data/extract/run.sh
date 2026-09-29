#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- data extract 'extract all ERROR lines' -f './fixtures/data/extract/server.log'
cargo run -- data extract 'extract all failed transactions' -f './fixtures/data/extract/transactions.csv'
cargo run -- data extract 'extract all database connection settings' -f './fixtures/data/extract/config.env'
cargo run -- data extract 'find lines containing NullReferenceException' -f './fixtures/data/extract/crash_report.txt'
cargo run -- data extract 'find all CRITICAL errors' -f './fixtures/data/extract/server.log'
