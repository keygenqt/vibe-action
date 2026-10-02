# Val Candidates

Val candidates are the data-binding mechanism in each action step. They
resolve source tags into `{name}` placeholders that the `action` template
expands.

## Schema

```yaml
val:
  - name: content
    data: query_raw
    when: 'is:path'
    mods: 'fetch|text'
  - name: content
    data: query_raw
    fail: 'is:empty:not'
    each: true
```

| Field  | Type           | Required | Description                                                      |
| ------ | -------------- | -------- | ---------------------------------------------------------------- |
| `name` | string         | yes      | Placeholder name used as `{name}` in `action`                    |
| `data` | string         | no       | Source tag or literal value                                      |
| `mods` | string         | no       | Operator pipe: `op:arg\|op:arg\|...` applied left to right       |
| `when` | string         | no       | Pre-mods soft guard (inspect operators); skip candidate if false |
| `fail` | string         | no       | Post-mods hard guard (inspect operators); bail if false          |
| `each` | bool or object | no       | Fan-out config for list data (split/merge)                       |

## Resolution order

Candidates are evaluated in list order. For each unique `name`, the
**first candidate whose `when` passes** (or has none) wins. Subsequent
candidates with the same `name` are skipped.

If no candidate wins for a name, the pipeline aborts with an error —
add a fallback candidate (no `when`) or declare the skip with the
action-level `off` guard, see [Pipeline — YAML Format](./pipeline-yaml.md).

## Data sources

`data` references a tag from one of these namespaces:

| Source                  | Example            | Notes                          |
| ----------------------- | ------------------ | ------------------------------ |
| Query provider          | `query_raw`        | Reserved `query_*` prefix      |
| System provider         | `system_language`  | Reserved `system_*` prefix     |
| CLI argument            | `arg_dry_run`      | The argument's own name        |
| Another action's result | `tag_project_path` | The referenced action's `tag`  |
| _(no data)_             | screenshot         | Mods-only candidate (no input) |

Only `query_*` and `system_*` are reserved prefixes. Argument names and
action tags are free-form; the `arg_` and `tag_` prefixes shown in the
examples are common conventions, not rules.

When `data` references another action's tag, a dependency is created.
The engine resolves actions in topological order — the referenced action
must complete before this one runs.

## Guards

### `when` — soft guard (skip)

Evaluated **before** `mods` are applied. If the result is falsy, the
candidate is skipped (not an error). Use to avoid unnecessary work,
especially for side-effect operators like `screenshot`:

```yaml
val:
  - name: content
    data: query_raw
    when: 'is:path' # only resolve if input is a file path
    mods: 'fetch|text'
  - name: content
    data: query_raw # fallback: use raw text
    fail: 'is:empty:not' # hard error if empty
```

### `fail` — hard guard (bail)

Evaluated **after** `mods` are applied. If the result is falsy, the
pipeline aborts with an error. Use to enforce that a value is valid:

```yaml
val:
  - name: content
    data: query_raw
    fail: 'is:empty:not' # bail if content is empty
```

Both `when` and `fail` accept **inspect operators only** — see
[Operators — Inspect](./operators-inspect.md).

## Mods

Operator pipe applied left to right. Each operator receives the output
of the previous one. The initial value comes from `data`.

```yaml
mods: 'fetch|text'                    # fetch URL/file, then extract text
mods: 'resolve:dir|scan|ast|format:json'  # resolve dir, scan, parse AST, format
mods: 'split|filter:eq:-|uniq|join'  # split lines, drop "-", deduplicate, join
```

Operators are **not** inline in the `action` template — they live in
`mods`. See [Read](./operators-read.md),
[Inspect](./operators-inspect.md),
[Transform](./operators-transform.md), and
[Write](./operators-write.md).

Operator arguments can reference other tags via `{name}` interpolation
(e.g. `mods: 'file:{tag_path}'`) — see
[Placeholders & Escapes](./placeholders.md).

## each — fan-out

When `each` is set, the resolved value is split into items, and the
`action` template is expanded once per item. Results are merged back
into a single value.

### Boolean form

```yaml
each: true
```

Equivalent to `split: '\n'` / `merge: '\n'` — splits by newline, merges
with newline.

### Object form

```yaml
each:
  split: '\n'
  merge: '\x1F'
```

| Field   | Description                             |
| ------- | --------------------------------------- |
| `split` | Separator to split the value into items |
| `merge` | Separator to join results back          |

Both support escape mnemonics (`\n`, `\t`, `\s`, `\x1F`).

### How fan-out works

1. Resolve the candidate value (after `mods`).
2. Split the value by `each.split` → list of items.
3. For each item, substitute `{name}` in the `action` template.
4. Execute each expanded action independently.
5. Join all results with `each.merge` → single string.
6. Store under the action's tag.

Example — process each changed file independently:

```yaml
val:
  - name: file
    data: tag_changed_files
    mods: 'trim'
    each:
      split: '\n'
      merge: '\x1F'
action: |
  cd {path}
  git diff HEAD -- {file} 2>/dev/null | head -c 6000
```

## Dead tags

A **dead tag** is an action skipped by the engine: its tag resolves to
an empty string, and downstream actions that reference it receive `""`.
The only source of a dead tag is the action-level `off` guard — see
[Pipeline — YAML Format](./pipeline-yaml.md).

```yaml
# Skipped in dry-run mode: no execution, no `ask` confirmation
- tag: tag_commit_exec
  run: cmd
  ask: true
  off:
    data: arg_dry_run
    when: 'equals:true'
  val:
    - name: path
      data: tag_project_path
    - name: msg
      data: tag_commit_message
  action: cd {path} && git add . && git commit -m {msg}
```

When `arg_dry_run` is `true`, the guard passes → dead tag → action
skipped. Downstream `{tag_commit_exec}` placeholders expand to an empty
string.
