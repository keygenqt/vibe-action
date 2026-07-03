# Vibe Action

Command router for shell and LLM tasks via YAML pipelines.

## Why

- ⚡ **Complex pipelines** — chain shell and LLM into one command
- 🔗 **Tag system** — auto-dependency graph via `{tag}`
- 🔧 **Modifiers** — 20+ inline value transformations
- 🔀 **When/Then** — conditional execution in YAML
- 🖥️ **System tags** — `{system_clipboard}`, `{system_os}` and more
- 👁️ **Vision** — screenshot description, person identification
- 🌐 **Fetch** — load and summarize web pages, PDFs, images
- 🤖 **Batch LLM** — role-based routing (tiny, small, medium, large, vision)
- 🎯 **CLI-first** — no browser, just terminal
- 🦀 **Fast** — built in Rust

## Quick Start

```bash
cargo install vibe-action

vibe-action
vibe-action faq -q "какие команды есть и зачем?"
vibe-action faq -q "как использовать модификаторы?"
```

## IDE Integration

- [VS Code](https://vibe-action.keygenqt.com/docs/ide-vscode.html) — Tasks + Task Notifier
- [IntelliJ IDEA](https://vibe-action.keygenqt.com/docs/ide-intellij.html) — External Tools

## Configuration

`~/.vibe-action/config.yaml`:

```yaml
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

## Install

```bash
cargo install vibe-action
```

## Supported Platforms

- **macOS** — full support
- **Linux** — full support

## Dependencies

- [Ollama](https://ollama.com), [DeepSeek](https://deepseek.com), or [Qwen](https://qwen.ai)
- [terminal-notifier](https://github.com/julienXX/terminal-notifier) (macOS, optional)
