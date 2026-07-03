# Introduction

Vibe Action is a command router that executes shell commands and LLM prompts via simple YAML pipelines.

## Why Vibe Action

- ⚡ **One command = complex pipeline** — chain shell scripts and LLM calls into a single action
- 🔗 **Tag system** — connect steps via `{tag}` references with automatic dependency graph
- 🔧 **Modifiers** — 20+ inline value transformations with arguments
- 🔀 **When/Then** — conditional execution in YAML without shell scripts
- 🌳 **AST parsing** — `{tag|ast}` auto-detects language from file, `{tag|ast:rs}` for explicit
- 🖥️ **System tags** — 15 built-in tags: `{system_clipboard}`, `{system_dir_pwd}`, `{system_os}` and more
- 👁️ **Vision support** — screenshot description, person identification, image from URL or clipboard
- 🌐 **Fetch** — load and summarize web pages, PDFs, images via `load` and `text` modifiers
- 📦 **Scan** — scan codebase and export AST as structured JSON
- 🧪 **Benchmarks** — automatic testing of all actions with timing and output validation
- ⚡ **Action cache** — instant startup via snapshot-based validation
- 🤖 **Batch LLM** — parallel execution across cluster nodes with role-based routing (tiny, small, medium, large, vision)
- ✅ **Type-safe** — validate outputs with `expect: string | list` and regex `check`
- 🔔 **Notifications** — optional desktop notifications on completion
- 🔐 **Confirmations** — ask before executing dangerous commands
- 💬 **Self-documenting** — built-in `faq` command answers questions about Vibe Action itself
- 🎯 **CLI-first** — no browser, no context switching. Everything in the terminal
- 🔒 **Local & free** — open source, local models via Ollama. No subscriptions
- 🦀 **Fast** — built in Rust

## Key Concepts

### YAML Pipelines

Describe your workflow in YAML, not code:

```yaml
name: extract
about: Extract matching lines from text and logs
args:
  - name: file
    short: f
    input: string
    help: Path to the log or text file
  - name: query
    short: q
    input: string
    help: Extraction criteria (e.g., 'find all errors')
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
      {query}

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

### When/Then Conditions

Use `when/then` for conditional logic without shell scripts:

```yaml
- tag: tag_result
  run: cmd
  expect: string
  action:
    - when: '{tag_check|contains:DIRTY}'
      then: echo "{tag_content}"
    - when: '{tag_check|contains:CLEAR}'
      then: echo "No errors found."
```

### System Tags

Access environment context anywhere in your pipelines:

```yaml
- tag: tag_info
  run: value
  expect: string
  action: |
    User: {system_user}
    OS: {system_os}
    PWD: {system_dir_pwd}
    Date: {system_date}
```

## How It Works

1. **You write a YAML file** describing your workflow — steps, types, dependencies
2. **The engine parses it** and builds a dependency graph from `{tag}` references
3. **Steps execute in order** — shell commands run locally, LLM prompts go to your cluster
4. **Results are validated** against expected types and optional regex patterns
5. **Final output** is displayed on screen, copied to clipboard, or sent as notification
