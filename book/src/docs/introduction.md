# Introduction

Vibe Action is a command router that executes shell commands and LLM prompts defined in simple YAML pipelines.

## Why Vibe Action

- ⚡ **One command = complex pipeline** — chain shell scripts and LLM calls into a single action
- 🔗 **Tag system** — connect steps via data references with an automatic dependency graph
- 🧩 **Operators** — pipeline of value transformations in `mods` (read, inspect, transform, write)
- 🛡️ **Guards** — `when` / `fail` predicates and `reg` output validation, all in YAML
- 🖥️ **System tags** — runtime environment: OS, user, directories, time, and more
- 📥 **Unified input** — query tags handle text, files, images, and interactive prompts from CLI or IDE
- 👁️ **Vision support** — screenshot description, person identification, image from URL or clipboard
- 🤖 **Batch LLM** — parallel execution across cluster nodes with role-based routing (`tiny`, `small`, `medium`, `large`, `vision`)
- ⏱️ **Process guard** — a new run supersedes the previous one, keeping shared state predictable
- 🔔 **Notifications** — optional desktop notifications on completion
- 🔐 **Confirmations** — ask before executing dangerous commands
- 💬 **Self-documenting** — the built-in `faq` action answers questions about Vibe Action itself
- 🔌 **IDE integration** — the `api` block drives the VS Code and IntelliJ plugins
- 🔒 **Open & flexible** — open source. Local models via Ollama or cloud APIs (DeepSeek, Qwen, Kimi, Zhipu)
- 🦀 **Fast** — built in Rust

## Key Concepts

### YAML pipelines

Describe your workflow in YAML, not code. Each file is one CLI command:

```yaml
version: 0.0.2
name: extract
about: Extract matching lines from text and logs
api:
  output: dialog
  input: query_prompt
actions:
  - tag: llm_log
    run: large
    reg: '^[^\n]+$'
    val:
      - name: search
        data: query_raw
      - name: line
        data: arg_file
        mods: 'fetch|text'
        fail: 'is:empty:not'
        each: true
    action: |
      [Task]
      If the line matches the query — output the EXACT line unchanged.
      If it does not match — output only a single dash: "-"
      [Query]
      {search}

      [Line]
      {line}
```

Steps resolve their inputs from `val` candidates, execute in dependency order, and pass results forward by tag. See [Pipeline YAML](./pipeline-yaml.md).

### Val candidates

Every step declares where its data comes from: a tag, an argument, or a query. Candidates resolve in order — the first one whose guard passes wins. See [Val Candidates](./val-candidates.md).

### Operators

`mods` chains operators left to right: `fetch|text`, `split|filter:eq:-|uniq|join`. Guards use inspect operators only. See the operator pages: [Read](./operators-read.md), [Inspect](./operators-inspect.md), [Transform](./operators-transform.md), [Write](./operators-write.md).

## How It Works

1. You write a YAML file describing your workflow — steps, data sources, guards
2. The engine loads pipelines from the actions directory and builds a CLI command per file
3. Steps execute in dependency order — shell commands run locally, LLM prompts go to your cluster
4. Outputs are validated against optional `reg` patterns
5. The final result goes to the terminal, the editor, or the clipboard

Ready to try it? Head to [Getting Started](./getting-started.md).
