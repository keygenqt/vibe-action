# Modifiers

Modifiers transform tag values inline using the pipe syntax: `{tag|modifier}`. Some modifiers accept arguments: `{tag|modifier:argument}`.

## `join`

Collapses a list into a single string with newline separator.

| Syntax             | Description                       |
| ------------------ | --------------------------------- |
| `{tag\|join}`      | Join list elements with `\n`      |
| `{tag\|join:uniq}` | Join with `\n`, remove duplicates |

| Input             | `{tag\|join}` | `{tag\|join:uniq}` |
| ----------------- | ------------- | ------------------ |
| `["a", "b"]`      | `"a\nb"`      | `"a\nb"`           |
| `["a", "b", "a"]` | `"a\nb\na"`   | `"a\nb"`           |

## `trim`

Strips characters from ends of strings, or filters list elements.

| Syntax           | Description                                                                |
| ---------------- | -------------------------------------------------------------------------- |
| `{tag\|trim}`    | Strip whitespace from ends of string, empty list elements                  |
| `{tag\|trim:<>}` | Strip `<>` and whitespace from ends of string, list elements equal to `<>` |

| Input           | `{tag\|trim}` | `{tag\|trim:<>}` |
| --------------- | ------------- | ---------------- |
| `"  hello  "`   | `"hello"`     | `"hello"`        |
| `"<path>"`      | `"<path>"`    | `"path"`         |
| `"<->"`         | `"<->"`       | `"-"`            |
| `["", "a", ""]` | `["a"]`       | `["a"]`          |

## `upper`

Transforms text to UPPERCASE.

| Input     | Output    |
| --------- | --------- |
| `"hello"` | `"HELLO"` |

## `lower`

Transforms text to lowercase.

| Input     | Output    |
| --------- | --------- |
| `"HELLO"` | `"hello"` |

## Chaining

Modifiers are applied once per tag reference. For multiple transformations, chain through separate steps:

```yaml
- tag: tag_cleaned
  type: value
  action: '{tag_raw|trim:<>}'

- tag: tag_final
  type: value
  action: '{tag_cleaned|join:uniq}'
```
