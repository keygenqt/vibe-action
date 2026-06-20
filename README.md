# Vibe Action

Command router for shell and LLM tasks via YAML pipelines.

## Why Vibe Action

- ⚡ **Complex pipelines** — chain shell and LLM into one command
- 🔗 **Tag system** — auto-dependency graph via `{tag}`
- 🔧 **Modifiers** — rich set of modifiers with arguments for transforming values inline
- 🔀 **Switch/when-then** — conditional execution in YAML
- 🌳 **AST parsing** — `{tag|ast:rs}` extract code structure
- 🖥️ **System tags** — `{system_clipboard}`, `{system_pwd}`, `{system_os}` and more
- 🤖 **Batch LLM** — parallel execution across cluster nodes with role-based routing
- ✅ **Type-safe** — validate with types and regex
- 🔔 **Notifications** — optional system notifications on completion
- 🔐 **Confirmations** — approve before executing
- 🎯 **CLI-first** — no browser, just terminal
- 🔒 **Local & free** — Ollama, 3b models, no subscriptions
- 🦀 **Fast** — built in Rust

## Quick Start

```bash
# Install
cargo install vibe-action

# AI-powered commit
vibe-action commit

# Translate files
vibe-action translate -f README.md -l Russian

# Rewrite tone from clipboard
vibe-action tone

# Extract errors from logs
vibe-action extract -f app.log -q 'find errors'
```

## Built-in Actions

| Action       | Description                               |
| ------------ | ----------------------------------------- |
| `commit`     | AI-generated git commit message           |
| `extract`    | Extract matching lines from logs and text |
| `find`       | Semantic file search by meaning           |
| `mock`       | Generate mock data (JSON, YAML, CSV)      |
| `naming`     | Code naming suggestions                   |
| `regex`      | Generate regular expression patterns      |
| `spellcheck` | Fix spelling in text and files            |
| `synonyms`   | Technical synonyms for a word             |
| `tone`       | Rewrite text with professional tone       |
| `translate`  | Translate text and files                  |

## Configuration

`~/.vibe-action/config.yaml`:

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

## Environment Variables

| Variable           | Description                                              | Default                      |
| ------------------ | -------------------------------------------------------- | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file                                      | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory                                | `~/.vibe-action/actions/`    |
| `VIBE_LOG_TYPE`    | Output mode: `cli`, `plain`, `json`, `tracing`           | `cli`                        |
| `VIBE_TRACE_LEVEL` | Tracing level: `error`, `warn`, `info`, `debug`, `trace` | `info`                       |

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

- [Ollama](https://ollama.com), [DeepSeek](https://deepseek.com), or [Qwen](https://qwen.ai) for LLM inference
- [terminal-notifier](https://github.com/julienXX/terminal-notifier) (macOS) for desktop notifications
