# Introduction

Vibe Action is a command router that executes shell commands and LLM prompts defined in simple YAML actions.

## Why Vibe Action

- ⚡ **One command = complex pipeline** — chain shell scripts and LLM calls into a single action
- 🔗 **Tag system** — connect steps via `{tag}` references with automatic dependency graph
- 🔧 **Modifiers** — 20+ inline value transformations with arguments
- 🔀 **When/Then** — conditional execution in YAML without shell scripts
- 🌳 **AST parsing** — `{tag|ast}` auto-detects language from file, `{tag|ast:rs}` for explicit
- 🖥️ **System tags** — built-in tags: `{system_dir_pwd}`, `{system_os}`, `{system_user}` and more
- 📥 **Unified Input** — `{query}` tag seamlessly handles text, files, images, and interactive prompts from CLI or IDE
- 👁️ **Vision support** — screenshot description, person identification, image from URL or clipboard
- 🌐 **Fetch** — load and summarize web pages, PDFs, images via `load` and `text` modifiers
- 📦 **Scan** — scan codebase and export AST as structured JSON
- ⚡ **Action cache** — instant startup via snapshot-based validation
- 🤖 **Batch LLM** — parallel execution across cluster nodes with role-based routing (tiny, small, medium, large, vision)
- ✅ **Type-safe** — validate outputs with `expect: string | list` and regex `check`
- 🔔 **Notifications** — optional desktop notifications on completion
- 🔐 **Confirmations** — ask before executing dangerous commands
- 💬 **Self-documenting** — built-in `faq` action answers questions about Vibe Action itself
- 🎯 **CLI-first** — no browser, no context switching. Everything in the terminal
- 🔌 **IDE Integration** — built-in `api` block for seamless VS Code and IntelliJ plugin support
- ⏱️ **Process Guard** — new runs automatically supersede previous ones, keeping state predictable
- 🔒 **Open & Flexible** — open source. Use local models via Ollama or cloud APIs (DeepSeek, Qwen, Kimi, Zhipu)
- 🦀 **Fast** — built in Rust

## Key Concepts

### YAML Pipelines

Describe your workflow in YAML, not code:

```yaml
version: 0.0.1
name: extract
about: Extract matching lines from text and logs
args:
  - name: file
    short: f
    input: string
    help: Path to the log or text file
api:
  output: replace
  input: query|prompt
actions:
  - tag: tag_lines
    run: cmd
    expect: list
    action: cat {file}
  - tag: tag_content
    run: small
    expect: string
    action: |
      [Task]
      If the line matches the query — output the EXACT line unchanged.
      If it does not match — output only a single dash: "-"
      Do NOT skip lines. Process every line.
      [Query]
      {query|prompt}
      [Line]
      {tag_lines}
  - tag: tag_clean
    run: value
    expect: string
    action: '{tag_content|trim:-}'
  - tag: tag_extract
    run: value
    expect: string
    action: '{tag_clean|uniq|join}'
```

### When/Then

Conditional execution in pure YAML — the first matching `when` branch wins, no shell scripting needed. Full syntax and examples — [Action Structure](./action-structure.md).

### System Tags

Environment context (`{system_user}`, `{system_os}`, `{system_dir_pwd}`, …) is available in any step without CLI arguments. Full list — [System Tags](./system-tags.md).

## How It Works

1. You write a YAML file describing your workflow — steps, types, dependencies
2. The engine parses it and builds a dependency graph from `{tag}` references
3. Steps execute in order — shell commands run locally, LLM prompts go to your cluster
4. Results are validated against expected types and optional regex patterns
5. Final output is printed to the terminal, copied to clipboard, or sent as notification

Ready to try it? Head to [Getting Started](./getting-started.md).
