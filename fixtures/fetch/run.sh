#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

cargo run -- fetch -s 'https://keygenqt.github.io/aurora-cli/'
cargo run -- fetch -s 'https://pdfobject.com/pdf/sample.pdf'
