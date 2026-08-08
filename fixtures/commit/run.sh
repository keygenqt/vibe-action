#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

FIXTURE_DIR="./fixtures/commit/application"

# Ensure .git is restored to dot-git even if the app crashes or is interrupted
cleanup() {
    if [ -d "$FIXTURE_DIR/.git" ]; then
        mv "$FIXTURE_DIR/.git" "$FIXTURE_DIR/dot-git"
    fi
}
trap cleanup EXIT

# Rename dot-git to .git for the test
if [ -d "$FIXTURE_DIR/dot-git" ]; then
    mv "$FIXTURE_DIR/dot-git" "$FIXTURE_DIR/.git"
fi

# Run the commit action in dry-run mode using -d flag
cargo run -- commit -p "$FIXTURE_DIR" -d

# Cleanup trap will rename .git back to dot-git automatically
