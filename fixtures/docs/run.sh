#!/bin/bash

# Setup root directory
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

# 1. Specific operator usage
cargo run -- docs "How to use the 'contains' operator?"

# 2. Available actions and their purpose
cargo run -- docs "какие действия есть и зачем?"

# 3. How to use operators
cargo run -- docs "как использовать операторы?"

# 4. Tag system
cargo run -- docs "如何使用标签系统？"
