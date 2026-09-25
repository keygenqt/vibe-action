# Operators — Transform

Transform operators are **value → value** pure functions. They never
touch the outside world — no files, no network, no clipboard.

Most operators are **list-aware**: they adapt to the input kind.
If the input contains `ITEM_SEP` (`\x1F`), it's treated as a list;
otherwise as a scalar. The operator documentation below notes the
behavior for each kind.

## Scalar operators

These accept scalar input only. List input is rejected.

### `base64` — encode / decode

| Syntax          | Description             |
| --------------- | ----------------------- |
| `base64:encode` | Encode value to base64  |
| `base64:decode` | Decode base64 to string |

Decode failure → empty string (not an error).

```yaml
mods: 'base64:encode'
mods: 'base64:decode'
```

### `split` — split string to list

| Syntax        | Description                |
| ------------- | -------------------------- |
| `split`       | Split by newline (default) |
| `split:<sep>` | Split by separator         |

Accepts scalar only; always produces a list. Supports escape mnemonics
in the separator (`\n`, `\t`, `\s`).

```yaml
mods: 'split'           # split by newline
mods: 'split:,\s'       # split by comma-space
```

## List → scalar operators

### `join` — collapse list to string

| Syntax       | Description                 |
| ------------ | --------------------------- |
| `join`       | Join with newline (default) |
| `join:<sep>` | Join with custom separator  |

Supports escape mnemonics.

```yaml
mods: 'join'             # newline-separated
mods: 'join:,\s'         # comma-space separated
```

### `item` — Nth list element

| Syntax     | Description                         |
| ---------- | ----------------------------------- |
| `item:<N>` | Return element at index N (0-based) |

Negative N counts from the end (`item:-1` = last). Out of range →
empty string. On scalar input: returns the whole string if N is 0,
else empty.

```yaml
mods: 'item:0'           # first element
mods: 'item:-1'          # last element
```

### `size` — count / length

| Syntax | Description                             |
| ------ | --------------------------------------- |
| `size` | List → item count; scalar → byte length |

Always returns a numeric string.

```yaml
mods: 'size'
```

## List filter operators

### `filter` — keep / remove items by pattern

| Syntax           | Description                           |
| ---------------- | ------------------------------------- |
| `filter:<X>`     | Remove items containing X (substring) |
| `filter:eq:<X>`  | Remove items exactly equal to X       |
| `filter:not:<X>` | Keep items containing X (inverted)    |

```yaml
mods: 'split|filter:eq:-'         # remove items equal to "-"
mods: 'split|filter:not:error'    # keep items containing "error"
```

### `grep` — regex filter

| Syntax          | Description                 |
| --------------- | --------------------------- |
| `grep:<re>`     | Keep items matching regex   |
| `grep:<re>:not` | Remove items matching regex |

On scalar input: returns the whole string on match, else empty string.

```yaml
mods: 'split|grep:^(feat|fix)'       # keep conventional commit lines
mods: 'split|grep:^\\s*$:not'        # remove blank lines
```

## List sort / deduplicate operators

### `sort` — alphabetical sort

| Syntax      | Description              |
| ----------- | ------------------------ |
| `sort`      | Sort ascending (default) |
| `sort:asc`  | Sort ascending           |
| `sort:desc` | Sort descending          |

On scalar: sorts characters.

```yaml
mods: 'split|sort'
mods: 'split|sort:desc'
```

### `uniq` — deduplicate

| Syntax | Description                         |
| ------ | ----------------------------------- |
| `uniq` | Remove duplicate items / characters |

Preserves first occurrence order.

```yaml
mods: 'split|uniq'
```

### `reverse` — reverse order

| Syntax    | Description                                       |
| --------- | ------------------------------------------------- |
| `reverse` | List → reverse item order; scalar → reverse chars |

```yaml
mods: 'split|reverse'
```

## List slice operators

### `take` — first N elements

| Syntax     | Description                                  |
| ---------- | -------------------------------------------- |
| `take:<N>` | List → first N items; scalar → first N chars |

```yaml
mods: 'split|take:5'
```

### `tail` — last N elements

| Syntax     | Description                                |
| ---------- | ------------------------------------------ |
| `tail:<N>` | List → last N items; scalar → last N chars |

```yaml
mods: 'split|tail:3'
```

## Per-item operators

These apply independently to each list item (or the whole scalar).

### `lower` / `upper` — case conversion

| Syntax  | Description  |
| ------- | ------------ |
| `lower` | To lowercase |
| `upper` | To UPPERCASE |

```yaml
mods: 'split|lower'
mods: 'upper'
```

### `replace` — substring replacement

| Syntax                | Description             |
| --------------------- | ----------------------- |
| `replace:<from>:<to>` | Replace all occurrences |

```yaml
mods: 'replace:old:new'
```

### `trim` — trim whitespace / chars

| Syntax         | Description                                           |
| -------------- | ----------------------------------------------------- |
| `trim`         | Strip whitespace from ends; remove empty list items   |
| `trim:<chars>` | Strip whitespace + chars; remove items equal to chars |

```yaml
mods: 'trim'            # strip whitespace, drop blanks
mods: 'trim:#'          # strip whitespace and '#', drop items equal to '#'
```

### `strip` — remove Markdown fences

| Syntax  | Description                                                    |
| ------- | -------------------------------------------------------------- |
| `strip` | Remove ` ```lang\\n...\\n``` `, `~~~`, `` ` ``, `"""` wrappers |

Auto-detects the wrapper type. Returns the inner content, trimmed.

```yaml
mods: 'strip'
```

### `default` — fallback for empty

| Syntax        | Description                             |
| ------------- | --------------------------------------- |
| `default:<X>` | If value is empty → X, else passthrough |

Per-item: each empty list item is replaced with X.

```yaml
mods: 'default:N/A'
```

## Format conversion

### `format` — convert between data formats

| Syntax         | Description      |
| -------------- | ---------------- |
| `format:json`  | Convert to JSON  |
| `format:json5` | Convert to JSON5 |
| `format:yaml`  | Convert to YAML  |
| `format:toml`  | Convert to TOML  |

Auto-detects input format (tries JSON, JSON5, YAML, TOML in order).
List-aware: each item is parsed independently, then wrapped into an
array for JSON/JSON5 output, concatenated for YAML. TOML does not
support top-level arrays.

```yaml
mods: 'format:json'
mods: 'scan|ast|format:json'
```

## Escape mnemonics

Operator arguments support `\n`, `\t`, `\s` escapes (e.g. `join:,\s`).
Full escape system — see [Placeholders & Escapes](./placeholders.md).
