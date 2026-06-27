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
progress: hello (val)... 100% (1/1)
info: completed in 8.58ms
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

```bash
$ vibe-action disk
progress: disk (cmd)... 100% (1/1)
info: completed in 21.13ms
── success ───────────────────────────────────────────────────────────────────
Filesystem        Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s1s1   460Gi    12Gi    72Gi    14%    455k  760M    0%   /
──────────────────────────────────────────────────────────────────────────────
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

```bash
$ vibe-action greet -n Alice
progress: greeting (val)... 100% (1/1)
info: completed in 8.45ms
── success ──
Hello, Alice!
─────────────
```

## Using System Tags

System tags provide context from your environment:

```yaml
# ~/.vibe-action/actions/status.yaml
name: status
about: Show system info
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

Available system tags: `{system_clipboard}`, `{system_clipboard_image}`, `{system_pwd}`, `{system_os}`, `{system_user}`, `{system_home}`, `{system_date}`, `{system_time}`, `{system_pid}`, `{system_temp}`.

## LLM Call

```yaml
# ~/.vibe-action/actions/explain-concept.yaml
name: explain-concept
about: Explain a concept
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
      Explain this concept in simple terms. Keep it under 3 sentences.
      {query}
```

```bash
$ vibe-action explain-concept -q "Rust borrow checker"
progress: answer (small)... 100% (1/1)
info: completed in 6.63s
── success ──
The Rust borrow checker...
─────────────
```

## Vision Call

```yaml
# ~/.vibe-action/actions/photo-desc.yaml
name: photo-desc
about: Describe a photo from clipboard
clipboard: true
args:
  - name: image
    short: f
    input: path
    help: Path to photo
    default: '{system_clipboard_image}'
actions:
  - tag: tag_description
    run: vision
    expect: string
    action: Describe this image in detail.
```

```bash
$ vibe-action photo-desc
progress: description (vision)... 100% (1/1)
info: completed in 3.21s
── success ──
A white cat with gray markings...
─────────────
```

## Conditional Actions (When/Then)

Use `when/then` for conditional execution:

```yaml
# ~/.vibe-action/actions/safe-commit.yaml
name: safe-commit
about: Commit only if there are changes
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

Each `when` is evaluated in order. The first matching `then` executes.

## Multi-Step Pipeline

```yaml
# ~/.vibe-action/actions/summarize-file.yaml
name: summarize-file
about: Read a file and summarize it
args:
  - name: file
    short: f
    input: string
    help: File to summarize
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

```bash
$ vibe-action summarize-file -f README.md
progress: summary (small)... 100% (2/2)
info: completed in 7.74s
── success ──
Vibe Action is a command router...
─────────────
```

## Tips

- **Tag naming:** use `tag_` prefix for consistency with built-in actions
- **Dependencies:** the engine sorts steps by `{tag}` references, not YAML order
- **Validation:** add `check: ".+"` to ensure non-empty output
- **Debugging:** set `VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug` to see each step's input and output
- **Clipboard:** use `default: '{system_clipboard}'` to read from clipboard
- **Images:** use `default: '{system_clipboard_image}'` with `input: path` for vision flows
- **Notifications:** add `notify: true` to show desktop notification on completion
