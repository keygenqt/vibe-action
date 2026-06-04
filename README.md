# Vibe Action

AI-native command router. Execute shell commands and LLM prompts via simple YAML actions.
Just say what you want — it figures out the rest.

## Quick Start

```bash
# Install
cargo install vibe-action

# Initialize config
vibe-action srv validate

# Run your first action
vibe-action run "закомить"
```

## How It Works

Vibe Action matches your natural language command to a YAML pipeline and executes it:

```yaml
keys:
  - закомить
  - commit

trigger:
  tag: commit
  type: cmd
  expect: void
  action: "git commit -m '{commit_message}'"

actions:
  - tag: changed_files
    type: cmd
    expect: list<string>
    action: 'git diff --name-only'

  - tag: file_summaries
    type: llm
    expect: string
    action: "Describe changes for each file:\n{file_diffs}"

  - tag: commit_message
    type: llm
    expect: string
    action: "Combine into a conventional commit message:\n{file_summaries}"
```

## Features

- **Natural language** — type what you mean, not command syntax
- **Tag pipeline** — chain shell commands and LLM prompts via `{tag}` references
- **Type validation** — `bool`, `number`, `string`, `json`, `list<T>`
- **Regex matching** — validate results with regex patterns
- **Complexity routing** — auto-selects the right model for each task
- **Modular** — share YAML files like Homebrew formulas
