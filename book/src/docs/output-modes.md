# Output Modes

Vibe Action routes all output through a single strategy selected at startup.
No `println!` exists outside the output module — everything flows through
`print_template!` / `print_text!` macros.

## Selecting a mode

Set `VIBE_LOG_TYPE` before startup:

```text
VIBE_LOG_TYPE=json vibe-action code review main.rs
```

| Value     | Strategy  | Use for                             |
| --------- | --------- | ----------------------------------- |
| _(unset)_ | `cli`     | Interactive terminal (default)      |
| `cli`     | `cli`     | Interactive terminal                |
| `plain`   | `plain`   | CI, piped output, scripts           |
| `json`    | `json`    | IDE plugin / Kotlin-Compose UI      |
| `tracing` | `tracing` | Debugging, detailed structured logs |
| _(auto)_  | `test`    | Internal test mode (`VIBE_TEST=1`)  |

## Verbosity

`VIBE_TRACE_LEVEL` gates `debug` and `trace` messages in all modes
except `test`. All other kinds (`plain`, `info`, `success`, `warning`,
`error`, `progress`) always print. Default: `info`.

| Level   | `debug` | `trace` |
| ------- | ------- | ------- |
| `error` | hidden  | hidden  |
| `warn`  | hidden  | hidden  |
| `info`  | hidden  | hidden  |
| `debug` | shown   | hidden  |
| `trace` | shown   | shown   |

Exception: in `tracing` mode the variable also configures the
subscriber's EnvFilter, which filters `error`/`warn`/`info` events
by severity as well — e.g. `VIBE_TRACE_LEVEL=warn` hides `info`
messages only in this mode.

## Mode details

### `cli` — terminal

ANSI-colored messages with semantic labels:

- **error** — red bold `error:`
- **warning** — yellow bold `warning:`
- **info** — blue bold `info:`
- **debug** — dimmed cyan `debug:`
- **trace** — dimmed magenta `trace:`
- **progress** — cyan bold, carriage-return overwrite for percentages
- **success** — green framed block with Markdown rendering and syntax highlighting

Template placeholders support inline styling:
`{key|color|style}` — e.g. `{tag|bright_green|bold}`.

Success blocks render Markdown via `termimad` and highlight code via
`syntect` (base16-eighties dark theme). Language aliases are resolved
automatically (`arkts`→typescript, `csharp`→cs, `batch`→bat, etc.).

### `plain` — unformatted

No ANSI codes, no framing, no Markdown rendering. Errors and warnings
go to stderr; everything else to stdout. Use in CI pipelines or when
piping output to other tools.

### `json` — structured

Each message is a JSON object on stdout:

```json
{
  "level": "success",
  "value": {
    "message": "result text"
  }
}
```

Fields are the key-value pairs from the template. Outer Markdown code
fences are stripped from string values.

Clipboard write operators (`clipboard_text`, `clipboard_image`) are
no-ops in JSON mode — the plugin owns the buffer. Use `api.output:
clipboard` in the YAML manifest to have the plugin copy the result —
see [IDE Plugin](./ide-plugin.md).

Messages are tagged with an **export context** for plugin routing:

| Context   | Purpose                  |
| --------- | ------------------------ |
| `actions` | Action list / status     |
| `status`  | System status response   |
| `success` | Final action result      |
| `confirm` | User confirmation prompt |

### `tracing` — structured logs

Delegates to the `tracing` crate (`tracing::info!`, `tracing::debug!`,
etc.). Verbosity is controlled by `VIBE_TRACE_LEVEL` — see
[Verbosity](#verbosity).

### `test` — minimal

Only `error` and `success` pass through; all other kinds are no-ops.
Activated automatically when `VIBE_TEST=1`. Used for integration test
assertions.

## Message model

Every message is an `OutputMsg` with:

- **kind** — semantic level (`plain`, `info`, `success`, `warning`,
  `error`, `debug`, `trace`, `progress`)
- **template** — layout string with `{key}` placeholders
- **fields** — key-value map (JSON values)

The formatter resolves placeholders per mode:
CLI applies `{key|color|style}` styling; plain/json substitute values
without styling.
