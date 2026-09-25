# Operators — Inspect

Inspect operators are **value → bool** predicates. They return the string
`"true"` or `"false"` per item and are used in `when` and `fail` guards
on val candidates.

All inspect operators support the `:not` suffix to invert the result.

```yaml
val:
  - name: content
    data: query_raw
    when: 'is:empty:not' # proceed only if non-empty
    fail: 'contains:ERROR' # bail if the value contains ERROR
```

## Operators

### `contains` — substring test

| Syntax         | Description              |
| -------------- | ------------------------ |
| `contains:<X>` | True if value contains X |

```yaml
when: 'contains:TODO'
when: 'contains:error:not'    # true if value does NOT contain "error"
```

### `equals` — exact equality

| Syntax       | Description                    |
| ------------ | ------------------------------ |
| `equals:<X>` | True if value equals X exactly |

```yaml
when: 'equals:true'
when: 'equals:0:not'          # true if value is NOT "0"
```

### `matches` — regex test

| Syntax         | Description                     |
| -------------- | ------------------------------- |
| `matches:<re>` | True if value matches the regex |

Invalid regex is a hard error.

```yaml
when: 'matches:^[0-9]+$'          # true if value is digits only
when: 'matches:^(feat|fix):not'   # true if value does NOT start with feat/ or fix:
```

### `compare` — numeric comparison

| Syntax               | Description                             |
| -------------------- | --------------------------------------- |
| `compare:<mode>:<N>` | Compare value against numeric threshold |

Modes:

| Mode  | Meaning   |
| ----- | --------- |
| `gt`  | value > N |
| `lt`  | value < N |
| `gte` | value ≥ N |
| `lte` | value ≤ N |

- Non-numeric value → `"false"` (not an error).
- Non-numeric threshold → hard error.
- Unknown mode → hard error.

```yaml
when: 'compare:gt:0'         # true if value is a number greater than 0
when: 'compare:lte:100:not'  # true if value is NOT ≤ 100 (or non-numeric)
```

### `is` — type/presence predicate

| Syntax      | Description                          |
| ----------- | ------------------------------------ |
| `is:<kind>` | True if value matches the given kind |

Kinds:

| Kind    | Checks                               |
| ------- | ------------------------------------ |
| `empty` | Value is empty string                |
| `num`   | Value parses as a float              |
| `int`   | Value parses as an integer           |
| `bool`  | Value is `"true"` or `"false"`       |
| `url`   | Value parses as a valid URL          |
| `path`  | Value is an existing filesystem path |
| `json`  | Value parses as valid JSON           |

```yaml
when: 'is:empty:not'    # proceed only if value is non-empty
when: 'is:path'         # proceed only if value is an existing path
when: 'is:json:not'     # true if value is NOT valid JSON
```

## Inversion (`:not`)

All inspect operators accept `:not` as a suffix after the arg. It swaps
`"true"` ↔ `"false"`. Applied after the operator, not before:

```yaml
# Equivalent ways to express "not empty":
when: 'is:empty:not'
```

## Per-item evaluation

All inspect operators are per-item over lists. When the input is a list
(separated by `ITEM_SEP`), each item is evaluated independently, and the
results are joined back into a list of `"true"`/`"false"` strings.

This matters when `when`/`fail` guards evaluate list data — every item
must pass for the candidate to proceed.
