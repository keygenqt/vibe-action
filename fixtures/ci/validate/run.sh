#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

FIXTURE_DIR="./fixtures/ci/application"

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

# Pre-review validation: the diff contains a leftover TODO to be caught
cargo run -- ci validate "$FIXTURE_DIR"
