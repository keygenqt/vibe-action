# Vibe Action

Command router for shell and LLM tasks via simple YAML actions.

**Chat is where you think. Vibe Action is where you prep the context — with local models, so your chat stays clean and cheap.**

## Why

- ⚡ **Complex pipelines** — chain shell and LLM into one command
- 🔗 **Tag system** — auto-dependency graph via tag references
- 🧩 **Operators** — 30+ read/inspect/transform/write operators chained in `mods`
- 🗂️ **Action groups** — load commands from git repos or local dirs as `vibe-action <group> <action>`
- 🛡️ **Guards** — `when` skips, `fail` bails, `reg` validates output — all in YAML
- 📥 **Unified Input** — query tags handle text, files, images, and interactive prompts from CLI or IDE
- 🖥️ **System tags** — `system_os`, `system_user`, `system_dir_pwd` and more
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

vibe-action --help
vibe-action faq "what commands are available?"
vibe-action faq "how do operators work?"
vibe-action commit .
vibe-action translate README.md
```

[IDE Integration](https://vibe-action.keygenqt.com/docs/ide-plugin.html)

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
