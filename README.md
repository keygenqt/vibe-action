# Vibe Action

AI-native command router. Execute shell commands and LLM prompts via simple YAML actions.
Just say what you want — it figures out the rest.

## Quick Start

```bash
# Install
cargo install vibe-action

# Run your first action
vibe-action action commit -p .

# Available actions
vibe-action action --help
```

## How It Works

Describe your workflow in a YAML file, and vibe-action builds a pipeline of shell commands and LLM prompts connected via `{tag}` references.

```yaml
name: commit
output: tag_commit
about: AI-generated commit message
args:
  - name: path
    short: p
    expect: string
    required: true

actions:
  # 1. Get list of changed files
  - tag: tag_changed_files
    type: cmd
    expect: list<string>
    match: .+
    action: cd {path} && git diff --name-only

  # 2. Get diff for each file (loop)
  - tag: tag_file_diff
    type: cmd
    expect: list<string>
    action: cd {path} && git diff {tag_changed_files}

  # 3. LLM summarizes each file diff (loop)
  - tag: tag_file_summary
    type: llm
    expect: list<string>
    action: |
      [Task]
      Summarize the git diff below in strictly ONE sentence (max 10 words).
      Start with an action verb (Add, Fix, Refactor, Change).
      Write strictly in English.

      [Diff]
      {tag_file_diff}

      [Result]
      <summary>

  # 4. LLM combines summaries into a commit message (join)
  - tag: tag_commit_message
    type: llm
    expect: string
    action: |
      [Task]
      Summarize all summaries below into ONE short conventional commit message (max 10 words).
      Allowed types: feat, fix, docs, refactor, test, chore.
      Format strictly: <type>: <commit> (NO parentheses or scopes).

      [Summaries]
      {tag_file_summary|join}

      [Result]
      <type>: <commit>

  # 5. Execute git commit (with confirmation)
  - tag: tag_commit
    type: cmd
    expect: string
    confirm: true
    action: cd {path} && git add . && git commit -m '{tag_commit_message}'
```

## Features

- **YAML pipelines** — define complex workflows with shell commands and LLM prompts
- **Tag system** — connect steps via `{tag}` references with automatic dependency ordering
- **List expansion** — `list<string>` automatically loops over each element
- **Join modifier** — `{tag|join}` collapses lists into a single string for LLM prompts
- **Type validation** — `bool`, `number`, `string`, `list<T>` with automatic parsing
- **Regex matching** — validate outputs with regex patterns
- **Modifiers** — `|upper`, `|lower`, `|trim` for text transformation
- **Confirmations** — `confirm: true` asks for user approval before executing
- **Clipboard** — `clipboard: true` copies the result automatically
- **Complexity routing** — auto-selects the right model for each task
- **Modular** — share YAML files like Homebrew formulas
