# CLI Reference

## Environment Variables

| Variable           | Description                                                                                          | Default                      |
| ------------------ | ---------------------------------------------------------------------------------------------------- | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file                                                                                  | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory                                                                            | `~/.vibe-action/actions/`    |
| `VIBE_LOG_TYPE`    | Output mode: `cli`, `plain`, `json`, `tracing`, `test`                                               | `cli`                        |
| `VIBE_TRACE_LEVEL` | Tracing level: `error`, `warn`, `info`, `debug`, `trace` (only works when `VIBE_LOG_TYPE=tracing`)   | `info`                       |
| `VIBE_SKIP_LOCK`   | Disables the singleton guard allowing multiple instances to run in parallel (e.g., for API clusters) | Not set (guard enabled)      |

## Output Modes

| Mode      | Description                                                                    |
| --------- | ------------------------------------------------------------------------------ |
| `cli`     | ANSI colors, progress bar, framed results (default)                            |
| `plain`   | Result only, no formatting (for tests/CI)                                      |
| `json`    | JSON objects `{"level":"...","message":"..."}` (for IDE/extension integration) |
| `tracing` | Structured logs with timestamps and log levels (supports `VIBE_TRACE_LEVEL`)   |
| `test`    | Internal testing mode                                                          |

## Trace Levels

Note: `VIBE_TRACE_LEVEL` is only respected when `VIBE_LOG_TYPE` is set to `tracing`. Using it with other modes will cause an error.

| Level   | Description               |
| ------- | ------------------------- |
| `error` | Errors only               |
| `warn`  | Warnings and errors       |
| `info`  | Flow progress and results |
| `debug` | Detailed engine internals |
| `trace` | Maximum verbosity         |

## Commands

Execute a YAML-defined action directly:

```text
vibe-action <name> [query] [args...]

vibe-action commit .
vibe-action translate-small README.md -l Russian
vibe-action tone   # reads from clipboard if no query is provided
vibe-action --help
```

Full list of ready-to-use actions — [Built-in Actions](./built-in-actions.md).

## System Commands

```text
vibe-action status   # Show system status
vibe-action clean    # Remove all cache
vibe-action stop     # Stop all running processes
```

## Action Arguments

Each action defines its own arguments in YAML — see [Action Structure](./action-structure.md). Use `--help` to see available options:

```text
vibe-action commit --help
vibe-action extract --help
```

## Exit Codes

| Code  | Description                                                |
| ----- | ---------------------------------------------------------- |
| `0`   | Success                                                    |
| `1`   | Error (validation failed, shell command failed, LLM error) |
| `130` | Instance superseded by a newer run (auto-cancelled)        |

## Debug Mode

Set `VIBE_LOG_TYPE=tracing` and `VIBE_TRACE_LEVEL=debug` (or `trace`) for detailed logs:

```text
VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug vibe-action commit .
```

Shows:

- Original and resolved action text
- Shell command output
- LLM prompts and responses
