# Placeholders & Escapes

The placeholder system connects val candidates to action templates and
operator arguments. Two escape mechanisms handle literal braces and
YAML whitespace.

## Action templates

In the `action` field, `{name}` is replaced with the resolved value of
the matching val candidate:

```yaml
val:
  - name: content
    data: query_raw
action: |
  [Task]
  Process {content}
```

Every `{name}` must reference a declared val candidate name. Undeclared
names are a validation error.

### Shell quoting

For `run: cmd` actions, placeholder values are **automatically
shell-quoted** via `shell_words::quote`. You don't need to add quotes
manually:

```yaml
# Correct — engine handles quoting
action: cd {path} && git commit -m {msg}

# Wrong — double-quoting breaks the command
action: cd "{path}" && git commit -m "{msg}"
```

For `run: tiny`/`small`/`medium`/`large`/`vision`, values are substituted
as-is (no quoting).

## Operator argument interpolation

Operator arguments in `mods` can reference tags via `{name}`:

```yaml
mods: 'file:{tag_path}'
```

The engine replaces `{name}` with the current value of the referenced
tag. The tag must already be resolved — if not, it's an error (declare
the dependency via `data:` in a val candidate).

## Double-brace escape

When an LLM prompt needs literal braces (e.g. JSON templates, LaTeX),
use double braces to escape them:

| Written  | Resolves to   | Use case                                   |
| -------- | ------------- | ------------------------------------------ |
| `{name}` | value of name | Placeholder substitution                   |
| `{{X}}`  | `{X}`         | Literal braces in output and operator args |

### In action templates

`{{X}}` → `{X}` in the prompt sent to the LLM. This lets you include
JSON templates or other brace-based syntax without the engine treating
them as placeholders:

```yaml
action: |
  Output JSON: {{"name": "value", "count": 42}}
```

The LLM receives: `"name": "value", "count": 42`

### In operator arguments

Same rule: `{{X}}` → `{X}`. This is how you pass literal braces through
the operator pipe parser, which uses `|` and `:` as separators:

```yaml
# Pass a JSON-like string through replace
mods: 'replace:{{key}}:value'
# Resolves to: replace {key} → value
```

## Brace-escape in operator parsing

The operator pipe parser splits on `|` and `:`, but **ignores separators
inside braces**. This means `{X}` and `{{X}}` protect their contents
from being split:

```yaml
# The : inside braces is NOT treated as a name/arg separator
mods: 'replace:{from:with}:to'
# name = "replace", arg = "{from:with}:to"
# After unescape: arg = "from:with:to"
```

### Unescape rules for operator args

After splitting, each arg is unescaped in a single left-to-right pass:

| Input   | Output | Rule                                      |
| ------- | ------ | ----------------------------------------- |
| `{X}`   | `X`    | Unwrap — removes braces, escapes `:`/`\|` |
| `{{X}}` | `{X}`  | Reduce — keeps braces, escapes `:`/`\|`   |

Use **single braces** when you want the value without braces (most
common). Use **double braces** when the operator needs a literal
brace-wrapped value.

## Escape mnemonics

In operator arguments and `each` config, these sequences are expanded
after brace unescaping:

| Code | Expands to | Purpose                    |
| ---- | ---------- | -------------------------- |
| `\n` | Newline    | Line separator             |
| `\t` | Tab        | Tab character              |
| `\s` | Space      | Shields from YAML trimmers |

YAML trims trailing whitespace by default. `\s` prevents a trailing
space from being eaten:

```yaml
# Join with comma-space
mods: 'join:,\s'
# Result: "item1, item2, item3"
```

```yaml
# Split on tab
mods: 'split:\t'
```

```yaml
# each with escape mnemonics
each:
  split: '\n'
  merge: '\x1F'
```

## ITEM_SEP (`\x1F`)

Internally, list items are separated by `ITEM_SEP` (`\x1F`, ASCII Unit
Separator). This character never appears in normal text, so it's safe as
an internal delimiter.

- Cross-step list data is joined with newlines for display.
- The `split` operator produces `ITEM_SEP`-separated lists.
- The `join` operator collapses `ITEM_SEP`-separated lists back to
  strings.
- `ITEM_SEP` is sanitized to `\n` in final output.

In `each` config, `merge: '\x1F'` preserves list structure across
fan-out steps — each item's result stays separated for downstream list
operators in the next action.

## Quick reference

| Context                  | `{name}`                   | `{{X}}`              | `\n`/`\t`/`\s` |
| ------------------------ | -------------------------- | -------------------- | -------------- |
| Action template          | Placeholder substitution   | Literal `{X}` in LLM | Not expanded   |
| Operator arg             | Tag interpolation + unwrap | Reduce to `{X}`      | Expanded       |
| `each.split/merge`       | Not interpolated           | Not reduced          | Expanded       |
| `when`/`fail`/`off.when` | Tag interpolation + unwrap | Reduce to `{X}`      | Expanded       |
