#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- data extract 'extract all ERROR lines' -f './fixtures/extract/server.log'
cargo run -- data extract 'extract all failed transactions' -f './fixtures/extract/transactions.csv'
cargo run -- data extract 'extract all database connection settings' -f './fixtures/extract/config.env'
cargo run -- data extract 'find lines containing NullReferenceException' -f './fixtures/extract/crash_report.txt'
cargo run -- data extract 'find all CRITICAL errors' -f './fixtures/extract/server.log'
