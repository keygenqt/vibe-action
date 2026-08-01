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
- ⏱️ **Process Guard** — new runs auto-cancel previous ones
- 🔌 **IDE Integration** — built-in `api` block for VS Code & IntelliJ
- 🔒 **Open & Flexible** — local models via Ollama or cloud APIs (DeepSeek, Qwen, Kimi, Zhipu)
- 🎯 **CLI-first** — no browser, just terminal
- 🦀 **Fast** — built in Rust

## Quick Start

```bash
cargo install vibe-action

vibe-action
vibe-action faq -q "какие команды есть и зачем?"
vibe-action faq -q "как использовать модификаторы?"
```

[IDE Integration](https://vibe-action.keygenqt.com/docs/vibe-action-cross.html)

## Configuration

`~/.vibe-action/config.yaml`:

```yaml
action:
  system: 'You are Vibe Action — a CLI tool. Output ONLY the result.'
  retries: 2

cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:3b-instruct
    role: small
    timeout_secs: 30
    temperature: 0.0
    seed: 42
    num_ctx: 4096
    num_predict: 512
    parallel: 2

  - provider: zhipu
    host: https://open.bigmodel.cn/api/paas/v4
    model: glm-4-flash
    role: medium
    timeout_secs: 60
    temperature: 0.1
    seed: 42
    num_ctx: 8192
    num_predict: 2048
    api_key: sk-...
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

- [Ollama](https://ollama.com) (recommended), [DeepSeek](https://deepseek.com), [Qwen](https://qwen.ai), [Kimi](https://moonshot.cn), or [Zhipu](https://open.bigmodel.cn/)
- [terminal-notifier](https://github.com/julienXX/terminal-notifier) (macOS, optional)
