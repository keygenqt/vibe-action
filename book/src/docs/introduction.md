# Introduction

Vibe Action is a command router that executes shell commands and LLM prompts via simple YAML pipelines.

## Why Vibe Action

- ⚡ **One command = complex pipeline** — chain shell scripts and LLM calls into a single action
- 🔗 **Tag system** — connect steps via `{tag}` references with automatic dependency graph
- 🔧 **Modifiers** — `{tag|upper|trim|join}` transform values inline
- 🌳 **AST parsing** — `{tag|ast:rs}` extract code structure for smarter prompts
- 🤖 **Batch LLM** — parallel execution across all cluster nodes
- ✅ **Type-safe** — validate outputs with types and regex
- 🔐 **Confirmations** — ask before executing dangerous commands
- 🎯 **CLI-first** — no browser, no context switching. Everything in the terminal
- 🔒 **Local & free** — open source, local models via Ollama. No subscriptions
- 🦀 **Fast** — built in Rust

## Key Concepts

### YAML Pipelines

Describe your workflow in YAML, not code:

```yaml
name: extract
about: Extract matching lines from text and logs
check: null
clipboard: false
args:
  - name: file
    short: f
    expect: string
    help: Path to the log or text file
    default: null
  - name: query
    short: q
    expect: string
    help: Extraction criteria (e.g., 'find all errors')
    default: null
actions:
  - tag: tag_lines
    type: cmd
    expect: list<string>
    check: null
    confirm: false
    action: cat {file}
  - tag: tag_content
    type: llm
    expect: string
    check: null
    confirm: false
    action: |
      [Task]
      If the line matches the query — output the EXACT line unchanged.
      If it does not match — output only a single dash: "-"
      Do NOT skip lines. Process every line.

      [Query]
      {query}

      [Line]
      {tag_lines}
  - tag: tag_clean
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_content|trim:-}'
  - tag: tag_extract
    type: value
    expect: string
    check: null
    confirm: false
    action: '{tag_clean|join:uniq}'
```

## How It Works

1. **You write a YAML file** describing your workflow — steps, types, dependencies
2. **The engine parses it** and builds a dependency graph from `{tag}` references
3. **Steps execute in order** — shell commands run locally, LLM prompts go to your cluster
4. **Results are validated** against expected types and optional regex patterns
5. **Final output** is displayed on screen and copied to clipboard if enabled
