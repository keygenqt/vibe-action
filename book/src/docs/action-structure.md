# Action Structure

Each action is a YAML file in `~/.vibe-action/actions/`. The engine loads all `.yaml` files recursively and builds a CLI command for each one.

## Minimal Action

```yaml
name: hello
about: Say hello
check: null
clipboard: false
args: []
actions:
  - tag: tag_hello
    run: value
    expect: string
    check: null
    confirm: false
    action: Hello, World!
```

```bash
$ vibe-action hello
info: completed in 0.01s
── success ──
Hello, World!
─────────────
```

## Full Structure

```yaml
name: my-action # CLI subcommand name
about: Description # Help text
check: '^[a-z]+$' # Optional: regex validation for final output
clipboard: true # Optional: copy result to clipboard
args: # Optional: CLI arguments
  - name: input
    short: i
    expect: string
    help: Input text
    default: 'default' # Optional: makes argument non-required
actions: # Pipeline steps (executed in order of dependencies)
  - tag: tag_step1
    run: cmd # cmd | llm | value
    expect: string # void | bool | number | string | list<T>
    check: '^.+$' # Optional: regex validation for this step
    confirm: true # Optional: ask before executing
    action: echo "Hello {input}!"
```

## Action Types

| Type    | Description                        |
| ------- | ---------------------------------- |
| `cmd`   | Shell command executed in terminal |
| `llm`   | Prompt sent to LLM cluster         |
| `value` | Static string, no execution        |

## Expect Types

| Type           | Description                       |
| -------------- | --------------------------------- |
| `void`         | No output                         |
| `bool`         | true/false, yes/no, да/нет, 是/否 |
| `number`       | Integer or float                  |
| `string`       | Text (default)                    |
| `list<string>` | List of strings, triggers loop    |

## Execution Order

Actions are sorted by their `{tag}` dependencies, not by their order in the file.
The engine builds a dependency graph and executes in topological order.

```yaml
actions:
  - tag: tag_files # 1st — no dependencies
    run: cmd
    action: find . -name '*.rs'

  - tag: tag_summary # 2nd — depends on tag_files
    run: llm
    action: Summarize - {tag_files}
```

## Check (Regex Validation)

`check` validates output against a regex pattern. If the output doesn't match — the step fails with an error.

```yaml
- tag: tag_files
  run: cmd
  check: '.+' # Must be non-empty
  action: git diff --name-only
```

## Confirm

`confirm: true` asks the user for approval before executing. Useful for dangerous commands.

```yaml
- tag: tag_commit
  run: cmd
  confirm: true
  action: git commit -m "feat: something"
```
