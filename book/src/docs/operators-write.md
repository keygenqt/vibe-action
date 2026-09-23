# Operators — Write

Write operators are **value → world (pass-through)**. They emit data to
the outside world and return the input value unchanged, so downstream
operators in the chain continue to see the original value.

## Operators

### `clipboard_text` — copy text to clipboard

| Syntax           | Description                                   |
| ---------------- | --------------------------------------------- |
| `clipboard_text` | Copy value to system clipboard (pass-through) |

Outer Markdown code fences are stripped before copying. This ensures
clean text reaches the clipboard even when the pipeline result is
wrapped in a code block.

**JSON mode**: no-op — see [Output Modes](./output-modes.md).

```yaml
val:
  - name: result
    data: tag_output
    mods: 'clipboard_text'
```

### `clipboard_image` — copy image to clipboard

| Syntax            | Description                                              |
| ----------------- | -------------------------------------------------------- |
| `clipboard_image` | Copy base64 PNG image to system clipboard (pass-through) |

Input must be a base64-encoded image string.

**JSON mode**: no-op — see [Output Modes](./output-modes.md).

```yaml
val:
  - name: image
    data: tag_screenshot
    mods: 'clipboard_image'
```

### `file` — write value to file

| Syntax               | Description                     |
| -------------------- | ------------------------------- |
| `file:<path>`        | Write value to file (overwrite) |
| `file:<path>:append` | Append value to file            |

The path is unescaped for brace-escape sequences. Empty path is a hard
error. Write failure is a hard error.

Always passes the input value through, regardless of write outcome.

```yaml
# Overwrite
val:
  - name: saved
    data: tag_json
    mods: 'file:/tmp/output.json'

# Append
val:
  - name: log
    data: tag_entry
    mods: 'file:/tmp/log.txt:append'
```

## Pass-through behavior

Write operators always return their input unchanged:

```text
input → [write op] → input (side effect: write to clipboard/file)
```

This means write operators can appear anywhere in a `mods` chain, and
subsequent operators see the original value:

```yaml
# Save to file AND copy to clipboard
mods: 'file:/tmp/result.txt|clipboard_text'
```

See [Output Modes](./output-modes.md) for details.
