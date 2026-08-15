#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- faq "How to use the 'contains' modifier?"
cargo run -- faq "какие команды есть и зачем?"
cargo run -- faq "как использовать модификаторы?"
cargo run -- faq "如何使用标签系统？"
