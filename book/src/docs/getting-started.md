# Getting Started

## Prerequisites

- **Ollama** (recommended), or API keys for **DeepSeek**, **Qwen**, **Kimi**, or **Zhipu** — for LLM inference
- **Rust** (if building from source)

### Supported Platforms

- **macOS** — full support
- **Linux** — full support

### Install Ollama

```bash
# macOS / Linux
curl -fsSL https://ollama.com/install.sh | sh

# Pull models for different roles
ollama pull qwen2.5-coder:3b-instruct   # small
ollama pull qwen2.5-coder:7b-instruct   # medium
ollama pull qwen2.5-coder:14b-instruct  # large
ollama pull qwen2.5vl:7b                # vision
```

## Install Vibe Action

### Via Cargo (recommended)

```bash
cargo install vibe-action
```

### Build from source

```bash
git clone https://gitcode.com/keygenqt_vz/vibe-action.git
cd vibe-action
cargo build --release
```

## First Run

On first run, Vibe Action creates the config and default actions:

```bash
$ vibe-action --help
```

This creates:

- `~/.vibe-action/config.yaml` — configuration
- `~/.vibe-action/actions/` — 21 built-in actions

## Configure Cluster

Edit `~/.vibe-action/config.yaml` to point to your Ollama instance:

```yaml
version: '0.0.3'

action:
  system: 'You are Vibe Action — a CLI tool. Output ONLY the result.'
  retries: 2

cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:14b-instruct
    timeout_secs: 60
    temperature: 0.1
    seed: 42
    num_ctx: 4096
    num_predict: 2048
    parallel: 1
```

## Run Your First Actions

```bash
# AI-powered commit
vibe-action commit -p .

# Translate a file
vibe-action translate-small -f README.md
vibe-action translate-large -f README.md

# Rewrite tone from clipboard
vibe-action tone

# Extract errors from logs
vibe-action extract -f app.log -q "find all errors"

# Describe a screenshot
vibe-action describe -f screenshot.png

# Fetch and summarize a web page
vibe-action fetch -s https://example.com

# System report
vibe-action sysinfo

# Scan project codebase as JSON
vibe-action scan

# Ask about Vibe Action itself
vibe-action faq -q "как использовать модификаторы?"

# Check version and status
vibe-action status

# Clear all cache
vibe-action clean

# Stop all running processes
vibe-action stop

# Run benchmarks
vibe-action bench
vibe-action bench -a faq -v

# See all available actions
vibe-action --help
```

## Debug Mode

Set `VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug` to see what's happening under the hood:

```bash
VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug vibe-action commit -p .
```

Shows each pipeline step: original command, resolved template, and result.

## Next Steps

- [Action Structure](./action-structure.md) — learn the YAML format
- [Built-in Actions](./built-in-actions.md) — explore what's included
- [Custom Actions](./custom-actions.md) — write your own
- [Tag System](./tag-system.md) — understand {tag} references
- [Modifiers](./modifiers.md) — transform output with pipe modifiers
- [System Tags](./system-tags.md) — built-in environment variables
