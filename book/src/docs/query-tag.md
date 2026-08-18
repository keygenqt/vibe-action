# Query Tag

The `{query}` tag is the unified entry point for data in any pipeline. It replaces the need for explicit `--text` or `--file` arguments and works seamlessly across CLI and IDE environments.

## How It Works

The engine automatically scans your pipeline for the `{query}` tag. If found, it enables input reading.

- **CLI:** reads from a positional argument (`vibe-action my-action "text"`). If not provided, falls back to the system clipboard.
- **IDE:** the plugin fills `{query}` based on the `api.input` setting — see [IDE Integration](./vibe-action-cross.md).

## Query Types

You can specify the type of input using the pipe syntax: `{query|type}`.

| Type                      | CLI (argument / clipboard)                                                                                                                    | IDE (plugin)                 |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| `raw` (or just `{query}`) | Raw text from argument or clipboard                                                                                                           | Editor selection text        |
| `file_path`               | First existing file from argument or clipboard (plain path or `file://` URI); empty string if none                                            | Path to the current file     |
| `project_path`            | Argument path as is if it is a directory, otherwise walks up for `.git`, `Cargo.toml`, `package.json`, …; falls back to the current directory | Project root path            |
| `line`                    | First line of the argument or clipboard text                                                                                                  | Cursor line number           |
| `prompt`                  | Interactive prompt in the terminal                                                                                                            | IDE input dialog             |
| `image`                   | Image path from argument encoded to base64 PNG, otherwise clipboard image file or screenshot                                                  | Screenshot or selected image |

`file_path` returning an empty string on miss is what makes the `|empty:not` pattern below work.

## Usage Example

```yaml
version: 0.0.1
name: my-action
api:
  output: replace
  input: query # Tell IDE to pass selection to {query}
actions:
  - tag: tag_content
    run: cmd
    expect: string
    check: .+
    action:
      # If input is a valid file path, read it. Otherwise, treat as text.
      - when: '{query|file_path|empty:not}'
        then: cat "{query|file_path|load}"
      - when: '{query|empty:not}'
        then: echo "{query}"
      - when: 'true'
        then: echo "ERROR - No input provided"
```

## IDE Integration (`api` block)

Where the IDE takes `{query}` from, how it presents the result, and extra `api.args` inputs — full reference in [IDE Integration](./vibe-action-cross.md).
