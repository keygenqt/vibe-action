# Action Structure

Each action is a YAML file in `~/.vibe-action/actions/`. The engine loads all `.yaml` files recursively and builds a CLI command for each one.

## Minimal Action

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

```text
$ vibe-action hello
info: completed in 0.01s
── success ──
Hello, World!
─────────────
```

## Full Structure

```yaml
version: 0.0.1 # Flow version (bump to force refresh)
name: my-action # CLI subcommand name
about: Description # Help text
check: '^[a-z]+$' # Optional: regex validation for final output
notify: true # Optional: show system notification on completion
args: # Optional: CLI arguments (for settings and flags)
  - name: dry_run
    short: d
    input: bool # string | bool | number | path | list<string> | list<bool> | list<number> | list<path>
    help: Do not execute
    default: false # Optional: makes argument non-required
api: # Optional: IDE plugin integration
  output: replace # replace | clipboard | dialog
  input: query # query | query|file_path | query|prompt | etc.
actions: # Pipeline steps (executed in order of dependencies)
  - tag: tag_step1
    run: cmd # cmd | value | small | medium | large | vision | tiny
    expect: string # string | list. Omit for no expected output.
    check: '^.+$' # Optional: regex validation for this step
    confirm: true # Optional: ask before executing
    action: echo "Hello {query}!"
```

## Version

Built-in flows ship with a `version` equal to the engine's internal flow version. On startup, a built-in flow whose `version` is missing or different is reset to its default content — custom edits to default flows will be overwritten.

Custom flows set their own `version`; bump it to force the engine to refresh the flow.

## Action Types

| Type     | Description                        |
| -------- | ---------------------------------- |
| `cmd`    | Shell command executed in terminal |
| `value`  | Static string, no execution        |
| `tiny`   | Prompt sent to tiny models         |
| `small`  | Prompt sent to small models        |
| `medium` | Prompt sent to medium models       |
| `large`  | Prompt sent to large models        |
| `vision` | Prompt sent to vision models       |

## Argument Types (`input`)

| Type           | Description                         |
| -------------- | ----------------------------------- |
| `string`       | Text (default)                      |
| `bool`         | true/false flag                     |
| `number`       | Integer or float                    |
| `path`         | File path (validated for existence) |
| `list<string>` | Comma-separated list of strings     |
| `list<bool>`   | Comma-separated list of bool values |
| `list<number>` | Comma-separated list of numbers     |
| `list<path>`   | Comma-separated list of file paths  |

## Expect Types

| Type     | Description                    |
| -------- | ------------------------------ |
| `string` | Text (default)                 |
| `list`   | List of strings, triggers loop |

Omit `expect` for steps with no expected output.

## Conditional Actions (When/Then)

`action` can be a list of `when/then` pairs for conditional execution:

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

Each `when` condition is evaluated. The first matching `then` is executed.

If no condition matches — the step fails with an error.

## Execution Order

Steps are sorted by their `{tag}` dependencies, not by their order in the file. Dependency graph, list expansion and circular dependencies — [Tag System](./tag-system.md).

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

## IDE Plugin Integration (`api`)

The optional `api` block connects the action to IDE plugins: where to take `{query}` from and how to present the result. Full reference with all input sources — [IDE Integration](./vibe-action-cross.md).

## Clipboard

Copy a value mid-pipeline with the `clipboard` modifier — [Modifiers](./modifiers.md).
