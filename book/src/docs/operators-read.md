# Operators — Read

Read operators fetch data from the outside world into the pipeline.
They are **world → value**: no input validation beyond existence;
failure produces an `Err` (file not found, parse error, etc.).

Read operators are used in the `mods` field of a val candidate:

```yaml
val:
  - name: content
    data: query_raw
    mods: 'fetch|text'
```

## Operators

### `ast` — parse source code to JSON

Parses a source file into a structured JSON AST via `vibe-ast`.

| Syntax       | Description                                       |
| ------------ | ------------------------------------------------- |
| `ast`        | Brief output (auto-detect language)               |
| `ast:brief`  | Signatures only — no function/class bodies        |
| `ast:full`   | Full output with bodies and imports               |
| `ast:<lang>` | Explicit language code (overrides auto-detection) |

Supported language codes: `rs`, `py`, `ts`, `js`, `java`, `go`, `cs`,
`kt`, `swift`, `dart`, `sh`, `bat`, `ets`, `md`.

In `brief` mode, function/class bodies, imports, tags, markers, and
warnings are stripped. The `path` field is injected into the root JSON
object.

List-aware: processes each file path independently, returns one JSON
object per file. Empty result for a file (e.g. parse failure) is
dropped from the output.

### `fetch` — download URL or resolve local file

| Syntax  | Description                                          |
| ------- | ---------------------------------------------------- |
| `fetch` | URL → temp file path; existing local file → resolved |

- URL (`http://` / `https://`) → downloads to a temp file (deduped by
  URL hash, extension from MIME). Returns the temp file path.
- Existing local file → resolves to absolute path.
- Neither → passes the value through unchanged.

Per-item: works over list input.

### `resolve` — resolve path to absolute

| Syntax         | Description                             |
| -------------- | --------------------------------------- |
| `resolve`      | Resolve any path (`~`, `./`, `../`)     |
| `resolve:dir`  | Resolve only if the path is a directory |
| `resolve:file` | Resolve only if the path is a file      |

Returns an empty string if the path doesn't exist or doesn't match the
filter. Per-item.

### `scan` — scan directory for file paths

| Syntax | Description                                    |
| ------ | ---------------------------------------------- |
| `scan` | Scan directory tree, return list of file paths |

Input must be a directory path (resolves `~`, `./`, `../`). Returns
items separated by `ITEM_SEP` (`\x1F`). Errors if the path is not a
directory. List-aware across multiple directory paths.

### `screenshot` — interactive screen capture

| Syntax       | Description                                    |
| ------------ | ---------------------------------------------- |
| `screenshot` | Opens interactive area selection, returns path |

Triggers the OS-native screenshot tool for area selection. Returns the
saved image path (PNG). Ignores input value.

**Platform support:**

| OS              | Tools (tried in order)                                                    |
| --------------- | ------------------------------------------------------------------------- |
| macOS           | `screencapture -i -x`                                                     |
| Linux (Wayland) | Desktop-native first (`gnome-screenshot`, `spectacle`), then `grim+slurp` |
| Linux (X11)     | `scrot -s`, `maim -s`, `gnome-screenshot -a`, `import`                    |

If the user cancels the selection (Esc), the operator returns an error.
Temp screenshots older than 24 hours are cleaned up automatically.

### `text` — extract text from files or content

| Syntax | Description                                                  |
| ------ | ------------------------------------------------------------ |
| `text` | Extract text from HTML/PDF/image file, or base64 passthrough |

Processing order per item:

1. **Base64 image** → passed through as-is.
2. **HTML string** → converted to plain text (`html2text`).
3. **File path** (resolved) → tried in order:
   - Image bytes → base64 encoded.
   - PDF bytes → text extracted (`pdf_extract`).
   - HTML content → converted to plain text.
   - Otherwise → raw file content as UTF-8.

Errors if the file doesn't exist or the content type is unsupported.
Per-item.

## Chaining with other operators

Read operators typically appear at the start of a `mods` chain, fetching
data that downstream transform/inspect/write operators then process:

```yaml
# Fetch a URL, extract its text content
mods: 'fetch|text'

# Resolve a directory, scan it, parse AST, format as JSON
mods: 'resolve:dir|scan|ast|format:json'

# Read a file path, extract text, strip markdown fences
mods: 'fetch|text|strip'
```

See [Transform](./operators-transform.md) and
[Inspect](./operators-inspect.md) for downstream operators.
