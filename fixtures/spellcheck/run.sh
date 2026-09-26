#!/bin/bash

# Set root dir
cd "$( dirname -- "${BASH_SOURCE[0]}"; )/../../";

# 1. Dirty English text - tests basic spellcheck
cargo run -- text spellcheck './fixtures/spellcheck/dirty_english.txt'

# 2. Dirty Russian text - tests non-English spellcheck
cargo run -- text spellcheck './fixtures/spellcheck/dirty_russian.txt'

# 3. Dirty Markdown with code - tests formatting preservation
cargo run -- text spellcheck './fixtures/spellcheck/dirty_markdown.md'

# 4. Clean text - tests "No errors found" path
cargo run -- text spellcheck './fixtures/spellcheck/clean_english.txt'
