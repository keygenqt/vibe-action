#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- extract 'extract all ERROR lines' -f './fixtures/extract/server.log'
cargo run -- extract 'extract all failed transactions' -f './fixtures/extract/transactions.csv'
cargo run -- extract 'extract all database connection settings' -f './fixtures/extract/config.env'
cargo run -- extract 'find lines containing NullReferenceException' -f './fixtures/extract/crash_report.txt'
cargo run -- extract 'find all CRITICAL errors' -f './fixtures/extract/server.log'
