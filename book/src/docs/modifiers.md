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

## `upper` / `lower`

Transform text case.

| Modifier | Input     | Output    |
| -------- | --------- | --------- |
| `upper`  | `"hello"` | `"HELLO"` |
| `lower`  | `"HELLO"` | `"hello"` |

## `take`

Returns first N characters of a string or first N elements of a list.

| Syntax          | Description                    |
| --------------- | ------------------------------ |
| `{tag\|take:N}` | Take first N chars or elements |

| Input             | `{tag\|take:1}` | `{tag\|take:2}` |
| ----------------- | --------------- | --------------- |
| `"hello"`         | `"h"`           | `"he"`          |
| `["a", "b", "c"]` | `["a"]`         | `["a", "b"]`    |

## `split`

Splits a string into a list by newlines.

| Syntax         | Description                  |
| -------------- | ---------------------------- |
| `{tag\|split}` | Split string to list by `\n` |

| Input       | `{tag\|split}`    |
| ----------- | ----------------- |
| `"a\nb\nc"` | `["a", "b", "c"]` |

## `sort`

Sorts a list alphabetically.

| Syntax             | Description              |
| ------------------ | ------------------------ |
| `{tag\|sort}`      | Sort ascending (default) |
| `{tag\|sort:asc}`  | Sort ascending           |
| `{tag\|sort:desc}` | Sort descending          |

## `reverse`

Reverses a string or list.

| Syntax           | Description   |
| ---------------- | ------------- |
| `{tag\|reverse}` | Reverse order |

## `size`

Returns the length of a string or list as a number.

| Syntax        | Description        |
| ------------- | ------------------ |
| `{tag\|size}` | Number of elements |

## `resolve`

Resolves a path to absolute form (`~`, `.`, `..` expanded).

| Syntax           | Description              |
| ---------------- | ------------------------ |
| `{tag\|resolve}` | Resolve to absolute path |

## `ast`

Parses source code into structured JSON via [vibe-ast](https://crates.io/crates/vibe-ast). 14 languages supported.

| Syntax             | Description         |
| ------------------ | ------------------- |
| `{tag\|ast:rs}`    | Parse as Rust       |
| `{tag\|ast:py}`    | Parse as Python     |
| `{tag\|ast:ts}`    | Parse as TypeScript |
| `{tag\|ast:js}`    | Parse as JavaScript |
| `{tag\|ast:java}`  | Parse as Java       |
| `{tag\|ast:go}`    | Parse as Go         |
| `{tag\|ast:cs}`    | Parse as C#         |
| `{tag\|ast:kt}`    | Parse as Kotlin     |
| `{tag\|ast:swift}` | Parse as Swift      |
| `{tag\|ast:dart}`  | Parse as Dart       |
| `{tag\|ast:sh}`    | Parse as Bash       |
| `{tag\|ast:bat}`   | Parse as Batch      |
| `{tag\|ast:ets}`   | Parse as ArkTS      |
| `{tag\|ast:md}`    | Parse as Markdown   |

## Predicate Modifiers

Return `true`/`false` for use in `when` conditions. Support `:not` to invert.

| Modifier   | Description                    | Example                 |
| ---------- | ------------------------------ | ----------------------- |
| `contains` | String contains substring      | `{tag\|contains:error}` |
| `empty`    | String or list is empty        | `{tag\|empty}`          |
| `equals`   | String equals value            | `{tag\|equals:done}`    |
| `is_file`  | Path exists and is a file      | `{tag\|is_file}`        |
| `is_dir`   | Path exists and is a directory | `{tag\|is_dir}`         |

### Inverting with `:not`

Add `:not` to any predicate to invert the result:

| Syntax                  | Description              |
| ----------------------- | ------------------------ |
| `{tag\|empty:not}`      | String or list not empty |
| `{tag\|contains:x:not}` | Does not contain x       |
| `{tag\|is_file:not}`    | Path is not a file       |

## Chaining

Modifiers can be chained in a single expression:

```yaml
# Trim, uppercase, then join with deduplication
{tag_input|trim|upper|join:uniq}

# Split, take first 3, then join back
{tag_text|split|take:3|join}

# Check if not empty
{file|empty:not}
```
