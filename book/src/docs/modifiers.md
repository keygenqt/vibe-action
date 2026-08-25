# Modifiers

Modifiers transform tag values inline: `{tag|modifier}` or `{tag|modifier:argument}`.

All modifiers work with both strings and lists.

## Summary Table

| Modifier    | Syntax                        | Description                                                                                                  |
| ----------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `ast`       | `{tag\|ast}`                  | Parse source file to JSON AST, brief by default (auto-detects language)                                      |
| `ast`       | `{tag\|ast:brief}`            | Parse source file, brief output (signatures only, no bodies)                                                 |
| `ast`       | `{tag\|ast:full}`             | Parse source file, full output with bodies and imports                                                       |
| `ast`       | `{tag\|ast:rs}`               | Parse source code with explicit language (14 languages)                                                      |
| `clipboard` | `{tag\|clipboard}`            | Copy value to system clipboard (pass-through)                                                                |
| `clipboard` | `{tag\|clipboard:screenshot}` | Smart image fallback: get from clipboard, set value as image, or trigger interactive screen capture if empty |
| `contains`  | `{tag\|contains:X}`           | Check if contains substring (predicate, supports `:not`)                                                     |
| `empty`     | `{tag\|empty}`                | Check if string or list is empty (predicate, supports `:not`)                                                |
| `equals`    | `{tag\|equals:X}`             | Check if equals value (predicate, supports `:not`)                                                           |
| `format`    | `{tag\|format:json}`          | Convert between json, json5, yaml, toml formats                                                              |
| `is_dir`    | `{tag\|is_dir}`               | Check if path is a directory (predicate, supports `:not`)                                                    |
| `is_file`   | `{tag\|is_file}`              | Check if path is a file (predicate, supports `:not`)                                                         |
| `join`      | `{tag\|join}`                 | Join list with `\n`                                                                                          |
| `join`      | `{tag\|join:X}`               | Join list with custom separator                                                                              |
| `load`      | `{tag\|load}`                 | Fetch URL to temp file, resolve local path, or pass through base64                                           |
| `lower`     | `{tag\|lower}`                | Transform to lowercase                                                                                       |
| `resolve`   | `{tag\|resolve}`              | Resolve path to absolute form (`~`, `.`, `..`)                                                               |
| `reverse`   | `{tag\|reverse}`              | Reverse string or list order                                                                                 |
| `scan`      | `{tag\|scan}`                 | Scan directory and return list of file paths (via vibe-fs)                                                   |
| `size`      | `{tag\|size}`                 | Length of string or element count of list                                                                    |
| `sort`      | `{tag\|sort}`                 | Sort ascending (default)                                                                                     |
| `sort`      | `{tag\|sort:asc}`             | Sort ascending                                                                                               |
| `sort`      | `{tag\|sort:desc}`            | Sort descending                                                                                              |
| `split`     | `{tag\|split}`                | Split string to list by `\n`                                                                                 |
| `split`     | `{tag\|split:X}`              | Split string to list by separator X                                                                          |
| `strip`     | `{tag\|strip}`                | Auto-detect and remove common wrappers: markdown fences, tilde fences, inline backticks, triple quotes       |
| `take`      | `{tag\|take:N}`               | Take first N characters or elements                                                                          |
| `text`      | `{tag\|text}`                 | Extract text from HTML/PDF or convert image to base64                                                        |
| `trim`      | `{tag\|trim}`                 | Strip whitespace, drop empty list elements                                                                   |
| `trim`      | `{tag\|trim:chars}`           | Strip custom characters, drop matching list elements                                                         |
| `uniq`      | `{tag\|uniq}`                 | Remove duplicate characters (string) or elements (list)                                                      |
| `upper`     | `{tag\|upper}`                | Transform to UPPERCASE                                                                                       |

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

```text
{tag|empty:not}
{tag|contains:x:not}
{tag|is_file:not}
```

## Chaining

```text
{tag|uniq|trim|upper|join:,\s}
{tag|split|take:3|join}
```

## Escaping Literals

Literal braces (`{{name}}`, `{{tag|upper}}`) are parsed by the tag engine, not by modifiers — see [Tag System](./tag-system.md#escaping-literals).
