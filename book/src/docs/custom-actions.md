# Custom Actions

Create your own actions by adding `.yaml` files to `~/.vibe-action/actions/`. Any subdirectory works — the engine loads all files recursively.

Some examples below are fragments focused on a single feature — the full skeleton (`version`, `name`, `about`, `args`, `api`) lives in [Action Structure](./action-structure.md).

## Hello World

```yaml
version: 0.0.1
name: hello
about: Say hello
actions:
  - tag: tag_hello
    run: value
    expect: string
    action: Hello, World!
```

## Shell Command

```yaml
version: 0.0.1
name: disk
about: Show disk usage
actions:
  - tag: tag_disk
    run: cmd
    expect: string
    action: df -h /
```

## With Arguments

Arguments are used for settings and flags, not for the main data input.

```yaml
version: 0.0.1
name: greet
about: Greet someone
args:
  - name: name
    short: n
    input: string
    help: Name to greet
    default: World
actions:
  - tag: tag_greeting
    run: value
    expect: string
    action: Hello, {name}!
```

## LLM Call

Use the `{query}` tag to accept the main text input (from CLI argument or clipboard).

```yaml
actions:
  - tag: tag_answer
    run: small
    expect: string
    action: |
      Explain this concept in simple terms:
      {query}
```

## Vision Call

Use `{query|image}` to accept an image (from file path, URL, or clipboard).

```yaml
api:
  input: query|image
actions:
  - tag: tag_description
    run: vision
    expect: string
    action: |
      {query|image|load|text}
      Describe this image in detail.
```

## Fetch Web or PDF

Use `{query|load|text}` to fetch content from a URL or read a local file.

```yaml
actions:
  - tag: tag_description
    run: small
    expect: string
    action: |
      Summarize this document:
      {query|load|text}
```

## Scan Codebase

Use `{query|project_path}` to automatically determine the project root.

```yaml
version: 0.0.1
name: my-scan
about: Scan project codebase as JSON
api:
  input: query|project_path
actions:
  - tag: tag_validate
    run: cmd
    expect: string
    action:
      - when: '{query|project_path|is_dir}'
        then: echo "{query|project_path}"
      - when: '{query|project_path|is_dir:not}'
        then: echo "'{query|project_path}' is not a directory" && exit 1
  - tag: tag_resolve
    run: value
    expect: string
    action: '{tag_validate|resolve}'
  - tag: tag_ast
    run: value
    expect: list
    action: '{tag_resolve|scan|ast:brief}'
  - tag: tag_json
    run: value
    expect: string
    action: '{tag_ast|format:json}'
```

## Clipboard

Use the `clipboard` modifier to copy values at any pipeline step:

```yaml
- tag: tag_copy
  run: value
  expect: string
  action: '{tag_data|format:json|clipboard}'
```

## Conditional Actions (When/Then)

```yaml
actions:
  - tag: tag_changed
    run: cmd
    expect: list
    action: git diff --name-only
  - tag: tag_commit
    run: cmd
    expect: string
    action:
      - when: '{tag_changed|empty:not}'
        then: git add . && git commit -m "auto: updates"
      - when: '{tag_changed|empty}'
        then: echo "Nothing to commit."
```

## Multi-Step Pipeline

```yaml
actions:
  - tag: tag_content
    run: cmd
    expect: string
    action: cat {query|file_path}
  - tag: tag_summary
    run: small
    expect: string
    action: |
      Summarize this file in 2-3 sentences:
      {tag_content}
```

## Tips

- **Tag naming:** use `tag_` prefix for consistency with built-in actions
- **Dependencies:** the engine sorts steps by `{tag}` references, not YAML order — [Tag System](./tag-system.md)
- **Validation:** add `check: ".+"` to ensure non-empty output
- **Debugging:** `VIBE_LOG_TYPE` / `VIBE_TRACE_LEVEL` — [CLI Reference](./cli-reference.md)
- **Clipboard:** use `{tag|clipboard}` to copy any value to clipboard mid-pipeline
- **Images:** use `{query|image}` for vision flows — works with files, URLs, and clipboard
- **System tags:** see [System Tags](./system-tags.md) for all available environment variables
- **Notifications:** add `notify: true` to show desktop notification on completion
- **IDE integration:** the `api` block reference lives in [IDE Integration](./vibe-action-cross.md)
