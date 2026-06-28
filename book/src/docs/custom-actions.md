# Custom Actions

Create your own actions by adding `.yaml` files to `~/.vibe-action/actions/`. Any subdirectory works — the engine loads all files recursively.

## Hello World

```yaml
# ~/.vibe-action/actions/hello.yaml
name: hello
about: Say hello
actions:
  - tag: tag_hello
    run: value
    expect: string
    action: Hello, World!
```

```bash
$ vibe-action hello
── success ──
Hello, World!
─────────────
```

## Shell Command

```yaml
# ~/.vibe-action/actions/disk.yaml
name: disk
about: Show disk usage
actions:
  - tag: tag_disk
    run: cmd
    expect: string
    action: df -h /
```

## With Arguments

```yaml
# ~/.vibe-action/actions/greet.yaml
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

## Using System Tags

System tags provide context from your environment:

```yaml
actions:
  - tag: tag_info
    run: value
    expect: string
    action: |
      User: {system_user}
      OS: {system_os}
      PWD: {system_pwd}
      Date: {system_date}
```

Available system tags: `{system_arch}`, `{system_clipboard}`, `{system_clipboard_image}`, `{system_date}`, `{system_home}`, `{system_hostname}`, `{system_language}`, `{system_os}`, `{system_pid}`, `{system_pwd}`, `{system_shell}`, `{system_temp}`, `{system_time}`, `{system_user}`.

## LLM Call

```yaml
args:
  - name: query
    short: q
    input: string
    help: What to explain
actions:
  - tag: tag_answer
    run: small
    expect: string
    action: |
      Explain this concept in simple terms:
      {query}
```

## Vision Call

```yaml
args:
  - name: image
    short: f
    input: string
    default: '{system_clipboard_image}'
actions:
  - tag: tag_description
    run: vision
    expect: string
    action: |
      {image|load|text}

      Describe this image in detail.
```

## Fetch Web or PDF

```yaml
args:
  - name: source
    short: s
    input: string
actions:
  - tag: tag_description
    run: small
    expect: string
    action: |
      Summarize this document:
      {source|load|text}
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
    action: cat {file}

  - tag: tag_summary
    run: small
    expect: string
    action: |
      Summarize this file in 2-3 sentences:
      {tag_content}
```

## Tips

- **Tag naming:** use `tag_` prefix for consistency with built-in actions
- **Dependencies:** the engine sorts steps by `{tag}` references, not YAML order
- **Validation:** add `check: ".+"` to ensure non-empty output
- **Debugging:** set `VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug` to see each step's input and output
- **Clipboard:** use `default: '{system_clipboard}'` to read from clipboard
- **Images:** use `{image|load|text}` for vision flows — works with files, URLs, and clipboard
- **Notifications:** add `notify: true` to show desktop notification on completion
