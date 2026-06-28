# Modifiers

Modifiers transform tag values inline: `{tag|modifier}` or `{tag|modifier:argument}`.
All modifiers work with both strings and lists.

## Summary Table

| Modifier   | Syntax              | Description                                                   |
| ---------- | ------------------- | ------------------------------------------------------------- |
| `ast`      | `{tag\|ast:rs}`     | Parse source code to JSON AST (14 languages)                  |
| `contains` | `{tag\|contains:X}` | Check if contains substring (predicate, supports `:not`)      |
| `empty`    | `{tag\|empty}`      | Check if string or list is empty (predicate, supports `:not`) |
| `equals`   | `{tag\|equals:X}`   | Check if equals value (predicate, supports `:not`)            |
| `is_dir`   | `{tag\|is_dir}`     | Check if path is a directory (predicate, supports `:not`)     |
| `is_file`  | `{tag\|is_file}`    | Check if path is a file (predicate, supports `:not`)          |
| `join`     | `{tag\|join}`       | Join list with `\n`                                           |
| `join`     | `{tag\|join:X}`     | Join list with custom separator                               |
| `load`     | `{tag\|load}`       | Fetch URL or resolve local file path                          |
| `lower`    | `{tag\|lower}`      | Transform to lowercase                                        |
| `resolve`  | `{tag\|resolve}`    | Resolve path to absolute form (`~`, `.`, `..`)                |
| `reverse`  | `{tag\|reverse}`    | Reverse string or list order                                  |
| `size`     | `{tag\|size}`       | Length of string or element count of list                     |
| `sort`     | `{tag\|sort}`       | Sort characters (string) or elements (list) ascending         |
| `sort`     | `{tag\|sort:asc}`   | Sort ascending                                                |
| `sort`     | `{tag\|sort:desc}`  | Sort descending                                               |
| `split`    | `{tag\|split}`      | Split string to list by `\n`                                  |
| `split`    | `{tag\|split:X}`    | Split string to list by separator X                           |
| `take`     | `{tag\|take:N}`     | Take first N characters or elements                           |
| `text`     | `{tag\|text}`       | Extract text from HTML/PDF or convert image to base64         |
| `trim`     | `{tag\|trim}`       | Strip whitespace, drop empty list elements                    |
| `trim`     | `{tag\|trim:chars}` | Strip custom characters, drop matching list elements          |
| `uniq`     | `{tag\|uniq}`       | Remove duplicate characters (string) or elements (list)       |
| `upper`    | `{tag\|upper}`      | Transform to UPPERCASE                                        |

## Escape Mnemonics

Use these codes in modifier arguments to bypass YAML whitespace trimming:

| Code | Description |
| ---- | ----------- |
| `\n` | Newline     |
| `\t` | Tab         |
| `\s` | Space       |

Example: `{tag|join:,\s}` → `", "` (comma-space).

## Predicate Modifiers

Return `"true"`/`"false"` as strings for `when` conditions. Invert with `:not`:

```yaml
{tag|empty:not}
{tag|contains:x:not}
{tag|is_file:not}
```

## Chaining

```yaml
{tag|uniq|trim|upper|join:,\s}
{tag|split|take:3|join}
{tag|empty:not}
```

## Escaping Literals

Use `{{...}}` to include literal braces:

```yaml
# Literal, not a tag:
{{name}}
{{tag|upper}}
```
