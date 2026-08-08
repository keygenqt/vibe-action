#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- faq -q "How to use the 'contains' modifier?"
cargo run -- faq -q "какие команды есть и зачем?"
cargo run -- faq -q "как использовать модификаторы?"
cargo run -- faq -q "如何使用标签系统？"
