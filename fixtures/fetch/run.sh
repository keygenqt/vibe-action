#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- data fetch 'https://keygenqt.github.io/aurora-cli/'
cargo run -- data fetch 'https://pdfobject.com/pdf/sample.pdf'
