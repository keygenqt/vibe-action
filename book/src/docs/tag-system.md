# Tag System

Tags connect pipeline steps. When you write `{tag_name}` in an action, the engine replaces it with the output of the step that has `tag: tag_name`.

## Basic Example

```yaml
actions:
  - tag: tag_files
    run: cmd
    expect: list<string>
    action: ls *.rs

  - tag: tag_summary
    run: small
    expect: string
    action: Summarize these files - {tag_files|join}
```

Step `tag_summary` depends on `tag_files`. The engine runs `tag_files` first, then passes its output to `tag_summary`.
The `|join` modifier collapses the `list<string>` result into a single string — without it, `tag_summary` would execute once for each file in the list.

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

When a step expects `list<string>` and receives a list from a tag, the engine runs the action **for each element**:

```yaml
actions:
  - tag: tag_files
    run: cmd
    expect: list<string>
    action: git diff --name-only
    # Returns: ["main.rs", "lib.rs"]

  - tag: tag_diff
    run: cmd
    expect: list<string>
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
