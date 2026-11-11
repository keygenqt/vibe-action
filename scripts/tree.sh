#!/bin/bash

source "$(dirname "$0")/common.sh"
setup_root

# Print tree
tree --gitignore -I "fixtures|book|tests|Cargo.lock"
