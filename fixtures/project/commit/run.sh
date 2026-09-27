#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

FIXTURE_DIR="./fixtures/project/commit/application"

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

# Run the commit action in dry-run mode, passing project path as query
cargo run -- project commit "$FIXTURE_DIR" -d

# Cleanup trap will rename .git back to dot-git automatically
