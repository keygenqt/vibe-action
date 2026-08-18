# Getting Started

## Prerequisites

- Ollama (recommended), or API keys for DeepSeek, Qwen, Kimi, or Zhipu — for LLM inference
- Rust (if building from source)

## Supported Platforms

- macOS — full support
- Linux — full support

## Install Ollama

```text
# macOS / Linux
curl -fsSL https://ollama.com/install.sh | sh

# Pull models for different roles
ollama pull qwen2.5-coder:3b-instruct   # small
ollama pull qwen2.5-coder:7b-instruct   # medium
ollama pull qwen2.5-coder:14b-instruct  # large
ollama pull qwen2.5vl:7b                # vision
```

## Install Vibe Action

Via Cargo (recommended):

```text
cargo install vibe-action
```

Build from source:

```text
git clone https://gitcode.com/keygenqt_vz/vibe-action.git
cd vibe-action
cargo build --release
```

## First Run

On first run, Vibe Action creates the config and default actions:

```text
$ vibe-action --help
```

This creates:

- `~/.vibe-action/config.yaml` — configuration
- `~/.vibe-action/actions/` — built-in actions

## Configure Cluster

Edit `~/.vibe-action/config.yaml` to point to your Ollama instance:

```yaml
cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:14b-instruct
```

Full field reference — roles, timeouts, multi-node clusters — see [Configuration](./configuration.md).

## Run Your First Actions

```text
# AI-powered commit
vibe-action commit

# Translate a file
vibe-action translate-small README.md

# Describe a screenshot
vibe-action describe screenshot.png
```

Full list of ready-to-use actions — [Built-in Actions](./built-in-actions.md).
System commands (`status`, `clean`, `stop`) and environment variables — [CLI Reference](./cli-reference.md).

## Next Steps

- [Action Structure](./action-structure.md) — learn the YAML format
- [Query Tag](./query-tag.md) — unified input for CLI and IDE
- [Built-in Actions](./built-in-actions.md) — explore what's included
- [Custom Actions](./custom-actions.md) — write your own
- [Tag System](./tag-system.md) — understand {tag} references
- [Modifiers](./modifiers.md) — transform output with pipe modifiers
- [System Tags](./system-tags.md) — built-in environment variables
