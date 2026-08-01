# CLI Reference

## Environment Variables

| Variable           | Description                                                                                            | Default                      |
| ------------------ | ------------------------------------------------------------------------------------------------------ | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file                                                                                    | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory                                                                              | `~/.vibe-action/actions/`    |
| `VIBE_LOG_TYPE`    | Output mode: `cli`, `plain`, `json`, `tracing`, `test`                                                 | `cli`                        |
| `VIBE_TRACE_LEVEL` | Tracing level: `error`, `warn`, `info`, `debug`, `trace` (**Only works when `VIBE_LOG_TYPE=tracing`**) | `info`                       |
| `VIBE_SKIP_LOCK`   | Disables the singleton guard allowing multiple instances to run in parallel (e.g., for API clusters)   | _Not set (guard enabled)_    |

## Output Modes

| Mode      | Description                                                                    |
| --------- | ------------------------------------------------------------------------------ |
| `cli`     | ANSI colors, progress bar, framed results (default)                            |
| `plain`   | Result only, no formatting (for tests/CI)                                      |
| `json`    | JSON objects `{"level":"...","message":"..."}` (for IDE/extension integration) |
| `tracing` | Structured logs with timestamps and log levels (supports `VIBE_TRACE_LEVEL`)   |
| `test`    | Internal testing mode                                                          |

## Trace Levels

_Note: `VIBE_TRACE_LEVEL` is only respected when `VIBE_LOG_TYPE` is set to `tracing`. Using it with other modes will cause an error._

| Level   | Description               |
| ------- | ------------------------- |
| `error` | Errors only               |
| `warn`  | Warnings and errors       |
| `info`  | Flow progress and results |
| `debug` | Detailed engine internals |
| `trace` | Maximum verbosity         |

## Commands

Execute a YAML-defined action directly:

```bash
vibe-action <name> [args...]
vibe-action commit -p .
vibe-action translate-small -f README.md -l Russian
vibe-action tone   # reads from clipboard
vibe-action --help
```

System commands:

```bash
vibe-action clean              # Remove all cache and temp files
vibe-action status             # Show version and actions count
vibe-action stop               # Stop all running processes
vibe-action bench              # Run all benchmarks
vibe-action bench -a faq -v    # Run benchmarks for specific action with output
```

## Action Arguments

Each action defines its own arguments in YAML. Use `--help` to see available options:

```bash
vibe-action commit --help
vibe-action extract --help
```

## Exit Codes

| Code | Description                                                |
| ---- | ---------------------------------------------------------- |
| 0    | Success                                                    |
| 1    | Error (validation failed, shell command failed, LLM error) |
| 130  | Instance superseded by a newer run (auto-cancelled)        |

## Debug Mode

Set `VIBE_LOG_TYPE=tracing` and `VIBE_TRACE_LEVEL=debug` (or `trace`) for detailed logs:

```bash
VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug vibe-action commit -p .
```

Shows:

- Original and resolved action text
- Shell command output
- LLM prompts and responses
