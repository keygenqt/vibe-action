#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

cargo run -- docs "How to use the 'contains' operator?"
cargo run -- docs "какие действия есть и зачем?"
cargo run -- docs "как использовать операторы?"
cargo run -- docs "如何使用标签系统？"
