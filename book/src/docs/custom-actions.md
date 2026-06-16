# Custom Actions

Create your own actions by adding `.yaml` files to `~/.vibe-action/actions/`. Any subdirectory works — the engine loads all files recursively.

## Hello World

```yaml
# ~/.vibe-action/actions/hello.yaml
name: hello
about: Say hello
check: null
clipboard: false
args: []
actions:
  - tag: tag_hello
    type: value
    expect: string
    check: null
    confirm: false
    action: Hello, World!
```

```bash
$ vibe-action hello
progress: hello (val)... 100% (1/1)
info: completed in 8.58ms
── success ──
Hello, World!
───────────────
```

## Shell Command

```yaml
# ~/.vibe-action/actions/disk.yaml
name: disk
about: Show disk usage
check: null
clipboard: false
args: []
actions:
  - tag: tag_disk
    type: cmd
    expect: string
    check: null
    confirm: false
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
check: null
clipboard: false
args:
  - name: name
    short: n
    expect: string
    help: Name to greet
    default: World
actions:
  - tag: tag_greeting
    type: value
    expect: string
    check: null
    confirm: false
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

## LLM Call

```yaml
# ~/.vibe-action/actions/explain.yaml
name: explain
about: Explain a concept
check: null
clipboard: false
args:
  - name: query
    short: q
    expect: string
    help: What to explain
actions:
  - tag: tag_answer
    type: llm
    expect: string
    check: null
    confirm: false
    action: |
      Explain this concept in simple terms. Keep it under 3 sentences.
      {query}
```

```bash
$ vibe-action explain -q "Rust borrow checker"
progress: answer (llm)... 100% (1/1)
info: completed in 6.63s
── success ──────────────────────────────────────────────────────────────────────────────────────────────────────────
The Rust borrow checker is a part of the Rust programming language that ensures memory safety and prevents common
programming errors like null pointer dereferencing or data races. It works by analyzing how variables are borrowed
(accessed) and mutably borrowed (modified) throughout your code, ensuring that these accesses are always valid and do
not conflict with each other. This allows Rust to avoid the need for a garbage collector while still providing safety
guarantees similar to those found in languages with automatic memory management.
─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
```

## Multi-Step Pipeline

```yaml
# ~/.vibe-action/actions/summarize-file.yaml
name: summarize-file
about: Read a file and summarize it
check: null
clipboard: false
args:
  - name: file
    short: f
    expect: string
    help: File to summarize
actions:
  - tag: tag_content
    type: cmd
    expect: string
    check: null
    confirm: false
    action: cat {file}

  - tag: tag_summary
    type: llm
    expect: string
    check: null
    confirm: false
    action: |
      Summarize this file in 2-3 sentences:
      {tag_content}
```

```bash
$ vibe-action summarize-file -f README.md
progress: summary (llm)... 100% (2/2)
info: completed in 7.74s
── success ─────────────────────────────────────────────────────────────────────────────────────────────────────────────
Vibe Action is a command router that simplifies the execution of shell commands and large language model prompts using
YAML pipelines. It allows developers to describe their workflows declaratively in YAML, automatically handling execution
order, running shell commands, calling LLMs, validating results, and providing answers, making repetitive tasks like
committing code, translating files, and extracting errors from logs more efficient and less error-prone.
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
```

## Tips

- **Tag naming:** use `tag_` prefix for consistency with built-in actions
- **Dependencies:** the engine sorts steps by `{tag}` references, not YAML order
- **Validation:** add `check: ".+"` to ensure non-empty output
- **Debugging:** set `VIBE_DEBUG=1` to see each step's input and output
