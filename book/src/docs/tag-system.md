# Tag System

Tags connect pipeline steps. When you write `{tag_name}` in an action, the engine replaces it with the output of the step that has `tag: tag_name`.

## The `{query}` Tag

There is one special, built-in tag: `{query}`. It does not require a corresponding `tag: query` step in your pipeline. Instead, the engine automatically resolves it from user input (CLI argument, clipboard, or IDE context).

```yaml
actions:
  - tag: tag_translate
    run: small
    expect: string
    action: |
      Translate to Russian:
      {query}
```

You can also use modifiers with `{query}`, such as `{query|file_path}` to validate it as a file path, or `{query|image}` to read an image. See [Query Tag](./query-tag.md) for details.

## Basic Example

```yaml
actions:
  - tag: tag_files
    run: cmd
    expect: list
    action: ls *.rs

  - tag: tag_summary
    run: small
    expect: string
    action: Summarize these files - {tag_files|join}
```

Step `tag_summary` depends on `tag_files`. The engine runs `tag_files` first, then passes its output to `tag_summary`.
The `|join` modifier collapses the list result into a single string — without it, `tag_summary` would execute once for each file in the list.

## Automatic Dependency Ordering

You don't need to write steps in execution order. The engine:

1. Scans all actions for `{tag}` references
2. Builds a directed acyclic graph (DAG)
3. Sorts topologically using Kahn's algorithm
4. Detects circular dependencies and reports errors

```yaml
actions:
  # These can be in any order — the engine sorts them:
  - tag: tag_commit
    run: cmd
    action: git commit -m '{tag_message}'

  - tag: tag_files
    run: cmd
    action: git diff --name-only

  - tag: tag_message
    run: small
    action: Write a commit message for: {tag_files}
```

Execution order: `tag_files` → `tag_message` → `tag_commit`

## List Expansion

When a step expects `string` or runs `cmd` but receives a list from a tag, the engine runs the action **for each element**:

```yaml
actions:
  - tag: tag_files
    run: cmd
    expect: list
    action: git diff --name-only
    # Returns: ["main.rs", "lib.rs"]

  - tag: tag_diff
    run: cmd
    expect: list
    action: git diff {tag_files}
    # Runs twice: git diff main.rs, git diff lib.rs
    # Returns: ["diff for main", "diff for lib"]
```

## Multiple Dependencies

A step can reference multiple tags:

```yaml
- tag: tag_report
  run: small
  expect: string
  action: |
    Compare these two files:
    File A: {tag_file_a}
    File B: {tag_file_b}
```

The engine waits for both `tag_file_a` and `tag_file_b` before running `tag_report`.

## Escaping Literals

Use `{{...}}` to include literal braces that should not be parsed as tags:

```yaml
- tag: tag_example
  run: value
  expect: string
  action: |
    To reference a tag, use {{tag_name}} syntax in your action.
    The modifier {{tag|upper}} transforms text to UPPERCASE.
```

## Circular Dependencies

Circular references are detected at startup and reported as errors:

```yaml
# ❌ This will fail validation:
- tag: tag_a
  action: echo {tag_b}
- tag: tag_b
  action: echo {tag_a}
```

```
Error: Circular dependency detected involving tag: 'tag_a'
```

## Invalid Modifiers

Using a pipe `|` without specifying a modifier name (e.g., `{tag|}`) will cause a fatal validation error, and the pipeline will halt immediately. Always ensure modifiers are properly named (e.g., `{tag|upper}`) or remove the pipe.

````

Осталось написать файл `query-tag.md` (который мы добавили в `SUMMARY.md`). Давай я набросаю его содержание, чтобы закрыть вопрос с документацией?

```markdown
# Query Tag

The `{query}` tag is the unified entry point for data in any pipeline. It replaces the need for explicit `--text` or `--file` arguments and works seamlessly across CLI and IDE environments.

## How It Works

The engine automatically scans your pipeline for the `{query}` tag. If found, it enables input reading.
- **CLI**: Reads from a positional argument (`vibe-action my-action "text"`). If not provided, it falls back to the system clipboard.
- **IDE**: The plugin provides the context based on the `api.input` setting (e.g., editor selection, file path).

## Query Types (Modifiers)

You can specify the type of input using the pipe syntax: `{query|type}`.

| Tag                    | CLI Behavior (Clipboard/Arg)                          | IDE Behavior (Plugin)               |
| ---------------------- | ----------------------------------------------------- | ----------------------------------- |
| `{query}` or `{query\|raw}` | Raw text from argument or clipboard                   | Editor selection text               |
| `{query\|file_path}`       | Validates text as a local file path                   | Path to the current file            |
| `{query\|project_path}`    | Searches for project root (`.git`, `Cargo.toml`, etc) | Project root path                   |
| `{query\|line}`            | First line of the text                                | Cursor line number                  |
| `{query\|prompt}`          | Interactive prompt in terminal                        | IDE input dialog                    |
| `{query\|image}`           | Image from clipboard or file path (base64 PNG)        | Screenshot or selected image        |

## Usage Example

```yaml
name: my-action
api:
  output: replace
  input: query  # Tell IDE to pass selection to {query}

actions:
  - tag: tag_content
    run: cmd
    expect: string
    check: .+
    action:
      # If input is a valid file path, read it. Otherwise, treat as text.
      - when: '{query|file_path|empty:not}'
        then: cat "{query|file_path|load}"
      - when: '{query|empty:not}'
        then: echo "{query}"
      - when: 'true'
        then: echo "ERROR: No input provided"
````

## IDE Integration (`api.input`)

To control how the IDE plugin fills the `{query}` tag, use the `api` block:

```yaml
api:
  output: replace # replace | clipboard | dialog
  input: query|file_path # query | query|file_path | query|prompt | etc.
```

If `input` is omitted, it defaults to `query` (editor selection).
