# Query Tag

The `{query}` tag is the unified entry point for data in any pipeline. It replaces the need for explicit `--text` or `--file` arguments and works seamlessly across CLI and IDE environments.

## How It Works

The engine automatically scans your pipeline for the `{query}` tag. If found, it enables input reading.

- **CLI**: Reads from a positional argument (`vibe-action my-action "text"`). If not provided, it falls back to the system clipboard.
- **IDE**: The plugin provides the context based on the `api.input` setting (e.g., editor selection, file path).

## Query Types (Modifiers)

You can specify the type of input using the pipe syntax: `{query|type}`.

| Tag                         | CLI Behavior (Clipboard/Arg)                           | IDE Behavior (Plugin)        |
| --------------------------- | ------------------------------------------------------ | ---------------------------- |
| `{query}` or `{query\|raw}` | Raw text from argument or clipboard                    | Editor selection text        |
| `{query\|file_path}`        | Validates text as a local file path                    | Path to the current file     |
| `{query\|project_path}`     | Searches for project root (`.git`, `Cargo.toml`, etc.) | Project root path            |
| `{query\|line}`             | First line of the text                                 | Cursor line number           |
| `{query\|prompt}`           | Interactive prompt in terminal                         | IDE input dialog             |
| `{query\|image}`            | Image from clipboard or file path (base64 PNG)         | Screenshot or selected image |

## Usage Example

```yaml
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

The `api` block controls how the IDE plugin interacts with the flow:

```yaml
api:
  output: replace # replace | clipboard | dialog
  input: query|prompt # query | query|file_path | query|prompt | etc.
  args: # additional inputs beyond {query}
    file: query|file_path
```

- **`output`** — where the IDE applies the final result (`replace`, `clipboard`, `dialog`).
- **`input`** — defines the source for the `{query}` tag. Defaults to `query` (editor selection).
- **`args`** — optional additional arguments, each mapped to a query type. Used when a flow needs multiple inputs beyond the main `{query}` positional arg. Each key is an argument name, each value is a query type (e.g., `query|file_path`).
