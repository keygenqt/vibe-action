# Pipeline — YAML Format

Each action is a YAML file in `~/.vibe-action/actions/`. The file name
becomes the CLI subcommand. Custom files are loaded automatically
alongside the built-in defaults.

## Top-level fields

```yaml
version: '0.0.2'
name: my-action
about: Short description for CLI help
notify: true

args:
  - name: arg_format
    short: 'f'
    input: string
    default: json
    help: 'Output format'

api:
  output: replace
  input: query_raw
  args:
    arg_format: query_raw

actions:
  - tag: tag_result
    run: small
    val:
      - name: content
        data: query_raw
    action: |
      [Task]
      Do something with {content}
```

| Field     | Type   | Required | Description                                             |
| --------- | ------ | -------- | ------------------------------------------------------- |
| `version` | string | yes      | Pipeline schema version (must match `PIPELINE_VERSION`) |
| `name`    | string | yes      | Action name — becomes CLI subcommand                    |
| `about`   | string | yes      | Short description for `--help`                          |
| `notify`  | bool   | no       | Desktop notification on completion (default: false)     |
| `args`    | list   | no       | CLI argument definitions                                |
| `api`     | object | no       | IDE plugin metadata (ignored by CLI runtime)            |
| `actions` | list   | yes      | Ordered list of pipeline steps                          |

## Args

Each argument becomes a CLI flag and a val-candidate tag under its own
name. Convention: prefix names with `arg_` (e.g. `arg_file` → flag
`--arg_file`, tag `arg_file`).

```yaml
args:
  - name: arg_dry_run
    short: 'd'
    input: bool
    default: false
    help: 'Print message without executing'
```

| Field     | Type   | Required | Description                                                                         |
| --------- | ------ | -------- | ----------------------------------------------------------------------------------- |
| `name`    | string | yes      | Argument name; used verbatim as tag and `--<name>` flag (convention: `arg_` prefix) |
| `short`   | char   | no       | Short flag (single ASCII letter)                                                    |
| `input`   | string | yes      | `string`, `bool`, `number`, `path`                                                  |
| `default` | string | no       | Default value; makes the argument optional                                          |
| `help`    | string | no       | Help text for CLI usage                                                             |

## API block

Metadata for the IDE plugin: input source, output target, extra args.
Ignored by the CLI runtime. Field reference and semantics — see
[IDE Plugin](./ide-plugin.md).

## Actions

Each item is one pipeline step. Execution order is determined
automatically by data dependencies (topological sort), not by list
position.

```yaml
actions:
  - tag: tag_result
    run: medium
    reg: '.+'
    ask: true
    when: 'is:empty:not'
    val:
      - name: content
        data: query_raw
        mods: 'fetch|text'
        when: 'is:path'
    action: |
      [Task]
      Process {content}
```

| Field    | Type   | Required | Description                                                         |
| -------- | ------ | -------- | ------------------------------------------------------------------- |
| `tag`    | string | yes      | Unique identifier; referenced by other steps via `data`             |
| `run`    | string | yes      | Execution engine (see below)                                        |
| `val`    | list   | no       | Val candidates — see [Val Candidates](./val-candidates.md)          |
| `when`   | string | no       | Action-level guard (inspect operators); skip entire action if false |
| `reg`    | string | no       | Regex to validate step output; fails if no match                    |
| `ask`    | bool   | no       | Prompt user confirmation before execution (default: false)          |
| `action` | string | yes      | Template with `{name}` placeholders from val candidates             |

### Action-level `when`

An action-level `when` is evaluated before any candidates are resolved.
If it fails, the entire action is skipped (dead tag). Accepts **inspect
operators only** — the same set as candidate-level `when` and `fail`.

This is separate from candidate-level `when`, which controls individual
candidate selection. Use the action-level guard to skip an entire step
based on runtime conditions.

### Run types

| Value    | Engine                                                 |
| -------- | ------------------------------------------------------ |
| `value`  | Literal passthrough — no execution, template as-is     |
| `cmd`    | Shell command via `sh -c`; values are shell-quoted     |
| `tiny`   | LLM prompt — tiny model (1-3b)                         |
| `small`  | LLM prompt — small model (3-7b)                        |
| `medium` | LLM prompt — medium model (7-14b)                      |
| `large`  | LLM prompt — large model (14b+)                        |
| `vision` | LLM prompt with extracted base64 images (vision model) |

LLM sizes route to cluster nodes by `role` — see
[Configuration](./configuration.md).

### Output validation (`reg`)

If `reg` is set, the step's output is tested against the regex. No match
→ hard error, pipeline aborts. Use it to enforce output format:

```yaml
reg: '.+'           # must be non-empty
reg: '^[^\n]+$'     # must be a single line
reg: 'DIRTY|CLEAR'  # must be one of these words
```

## Reserved prefixes

Action tags and argument names must not use `query_*` or `system_*`
prefixes — those are reserved for providers. A bare `query` tag is also
forbidden; use `query_raw` instead.

Self-referencing (`data` points to the action's own `tag`) is a
validation error.

## Version management

The `version` field must match the engine's `PIPELINE_VERSION`. For
custom actions a mismatch is a validation error — bump `version` when
the pipeline schema changes.

Built-in files are instead overwritten with the embedded default on
version mismatch — see [Built-in Actions](./built-in-actions.md).
