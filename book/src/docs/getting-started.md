# Getting Started

## Prerequisites

- **Ollama** (recommended), **DeepSeek API key**, or **Qwen** — for LLM inference
- **Rust** (if building from source)

### Install Ollama

```bash
# macOS / Linux
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull qwen2.5-coder:14b-instruct
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
$ vibe-action action --help
```

This creates:

- `~/.vibe-action/config.yaml` — cluster configuration
- `~/.vibe-action/actions/` — 10 built-in actions

## Configure Cluster

Edit `~/.vibe-action/config.yaml` to point to your Ollama instance:

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

## Run Your First Actions

```bash
# AI-powered commit
vibe-action action commit -p .

# Translate a file
vibe-action action translate -f README.md -l Russian

# Extract errors from logs
vibe-action action extract -f app.log -q "find all errors"

# Direct prompt to cluster
vibe-action prompt "Explain Rust lifetimes"

# See all available actions
vibe-action action --help
```

## Debug Mode

Use `--debug` to see what's happening under the hood:

```bash
vibe-action --debug action commit -p .
```

Shows each pipeline step: original command, resolved template, and result.

## Next Steps

- [Action Structure](./action-structure.md) — learn the YAML format
- [Built-in Actions](./built-in-actions.md) — explore what's included
- [Custom Actions](./custom-actions.md) — write your own
- [Tag System](./tag-system.md) — understand {tag} references
- [Modifiers](./modifiers.md) — transform output with pipe modifiers
