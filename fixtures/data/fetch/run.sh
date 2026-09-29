#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- data fetch 'https://keygenqt.github.io/aurora-cli/'
cargo run -- data fetch 'https://pdfobject.com/pdf/sample.pdf'
