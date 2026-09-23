# Providers — Query

Query providers resolve `query_*` tags from user input — CLI positional
args, clipboard, or IDE plugin data. They are resolved once at pipeline
startup (before the engine loop) and stored in `input_tags`.

## Contract

- Each `query_*` provider receives the raw positional argument (if any).
- Non-empty result wins. Empty input → `""` — the val candidate is
  skipped and the next candidate for the same name is tried.
- Exception: `query_image` returns `Err` if the input is not a valid
  image (hard failure, not empty).
- Empty query value without a `when` guard is a startup error (ambiguous
  input). Add `when: 'is:empty:not'` to explicitly handle missing input.

Usage in a val candidate:

```yaml
val:
  - name: content
    data: query_raw
    when: 'is:path'
    mods: 'fetch|text'
  - name: content
    data: query_raw
    fail: 'is:empty:not'
```

## Tags

### `query_raw` — raw input, no validation

| Tag         | Behavior                                    |
| ----------- | ------------------------------------------- |
| `query_raw` | Positional arg as-is; empty if not provided |

No type checking, no path resolution. Use when the input might be text,
a path, a URL — anything.

### `query_prompt` — interactive prompt marker

| Tag            | Behavior                                        |
| -------------- | ----------------------------------------------- |
| `query_prompt` | Returns raw value; signals interactive fallback |

This is a **presence marker**, not a data provider. When `query_prompt`
appears in any val chain and the query slot is empty, the CLI triggers
an interactive terminal prompt. In IDE mode, the plugin opens an input
dialog.

The prompt itself fires at a fixed early point in the flow — not during
val resolution.

### `query_clipboard` — combined clipboard

| Tag               | Behavior                                                     |
| ----------------- | ------------------------------------------------------------ |
| `query_clipboard` | Combined clipboard content by priority: text → paths → image |

Reads clipboard content trying each type in order. Token-limited to the
largest `num_ctx` across all cluster nodes (prevents oversized clipboard
input from blowing context windows).

### `query_clipboard_text` — clipboard text

| Tag                    | Behavior                    |
| ---------------------- | --------------------------- |
| `query_clipboard_text` | Raw text from the clipboard |

### `query_clipboard_path` — clipboard file paths

| Tag                    | Behavior                             |
| ---------------------- | ------------------------------------ |
| `query_clipboard_path` | Copied file paths from the clipboard |

Multiple paths are newline-separated.

### `query_clipboard_image` — clipboard image

| Tag                     | Behavior                             |
| ----------------------- | ------------------------------------ |
| `query_clipboard_image` | Clipboard image as base64 PNG string |

### `query_file_path` — file path from input

| Tag               | Behavior                                               |
| ----------------- | ------------------------------------------------------ |
| `query_file_path` | Explicit input → existing file path; `""` if not found |

Resolves the positional arg to an absolute path. Supports `file://` URI
format from IDE plugins. Returns `""` if no file exists at the path —
this makes the `when: 'is:empty:not'` pattern work for conditional
file-based candidates.

### `query_project_path` — project root path

| Tag                  | Behavior                                          |
| -------------------- | ------------------------------------------------- |
| `query_project_path` | Input → project root directory; `""` if not found |

Walks up from the input path looking for a project marker, then returns
the root directory. Recognized markers:

`.git`, `.hg`, `Cargo.toml`, `package.json`, `go.mod`, `pom.xml`,
`build.gradle`, `.idea`

Stops at the home directory or filesystem root. Returns `""` if no
marker is found.

### `query_line` — first line of input

| Tag          | Behavior                                    |
| ------------ | ------------------------------------------- |
| `query_line` | First line of the input text; `""` if empty |

Useful for single-line IDE inputs (cursor line, selection first line).

### `query_image` — image input

| Tag           | Behavior                                                     |
| ------------- | ------------------------------------------------------------ |
| `query_image` | Input (URL/file/base64) → validated base64; `Err` if invalid |

Accepts three input forms:

- **URL** (`http://` / `https://`) → downloads, encodes to base64.
- **File path** → reads file, encodes to base64.
- **Base64 string** → decoded and validated.

Always validates that the bytes form a real image. Invalid input is a
hard error — use a `when` guard or a fallback candidate to handle
missing images gracefully.

## Resolution flow

Query providers are resolved early, before the engine loop:

1. `apply_args()` — CLI flags → `input_tags`.
2. `apply_query_tags()` — each `query_*` in val candidates → `input_tags`.
3. `validate_query_tags()` — reject empty query values without a `when` guard.

After this, `input_tags` is frozen and the engine begins resolving
actions.

## CLI vs IDE

| Tag                  | CLI source                  | IDE source                   |
| -------------------- | --------------------------- | ---------------------------- |
| `query_raw`          | Positional arg or clipboard | Editor selection text        |
| `query_prompt`       | Interactive terminal prompt | IDE input dialog             |
| `query_file_path`    | Arg → existing file path    | Path to current file         |
| `query_project_path` | Arg → project root          | Project root path            |
| `query_line`         | First line of arg           | Cursor line                  |
| `query_image`        | Arg → URL/file/base64       | Screenshot or selected image |

The IDE plugin fills the query slot based on the `api.input` field in
the YAML manifest — see [IDE Plugin](./ide-plugin.md).
