# Vibe Action

Vibe Action is a command router that executes shell commands and LLM prompts via simple YAML pipelines.
Just say what you want — it figures out the rest.

## Why Vibe Action

- ⚡ **Complex pipelines** — chain shell and LLM into one command
- 🔗 **Tag system** — auto-dependency graph via `{tag}`
- 🤖 **Batch LLM** — parallel execution across cluster nodes
- ✅ **Type-safe** — validate with types and regex
- 🔐 **Confirmations** — approve before executing
- 🎯 **CLI-first** — no browser, just terminal
- 🔒 **Secure** — runs locally
- 🆓 **Free** — open source, local models
- 📦 **Modular** — share YAML like Homebrew formulas
- 🦀 **Fast** — built in Rust

## Quick Start

```bash
# Install
cargo install vibe-action

# AI-powered commit
vibe-action action commit

# Translate files
vibe-action action translate -f README.md -l Russian

# Extract errors from logs
vibe-action action extract -f app.log -q 'find all errors'

# Direct prompt to cluster
vibe-action prompt 'Explain Rust lifetimes'
```

## Built-in Actions

| Action       | Description                                |
| ------------ | ------------------------------------------ |
| `commit`     | AI-generated git commit message            |
| `extract`    | Extract matching lines from logs and text  |
| `find`       | Semantic file search by meaning            |
| `mock`       | Generate mock data (JSON, YAML, CSV, etc.) |
| `naming`     | Code naming suggestions                    |
| `regex`      | Generate regular expression patterns       |
| `spellcheck` | Fix spelling in text and files             |
| `synonyms`   | Technical synonyms for a word              |
| `tone`       | Rewrite text with professional tone        |
| `translate`  | Translate text and files                   |

## Configuration

`~/.vibe-action/config.yaml`:

```yaml
version: 0.0.1

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

## Install

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

## Dependencies

- Ollama, DeepSeek, or Qwen (for LLM cluster)
