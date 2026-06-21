# CLI Reference

## Environment Variables

| Variable           | Description                                              | Default                      |
| ------------------ | -------------------------------------------------------- | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file                                      | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory                                | `~/.vibe-action/actions/`    |
| `VIBE_LOG_TYPE`    | Output mode: `cli`, `plain`, `json`, `tracing`           | `cli`                        |
| `VIBE_TRACE_LEVEL` | Tracing level: `error`, `warn`, `info`, `debug`, `trace` | `info`                       |

## Output Modes

| Mode      | Description                                    |
| --------- | ---------------------------------------------- |
| `cli`     | ANSI colors, progress bar, framed results      |
| `plain`   | Result only, no formatting (for tests/CI)      |
| `json`    | JSON objects `{"level":"...","message":"..."}` |
| `tracing` | Structured logs with timestamps and log levels |

## Trace Levels

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
vibe-action translate-fast -f README.md -l Russian
vibe-action tone   # reads from clipboard
vibe-action --help
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

## Debug Mode

Set `VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug` for detailed logs:

```bash
VIBE_LOG_TYPE=tracing VIBE_TRACE_LEVEL=debug vibe-action commit -p .
```

Shows:

- Original and resolved action text
- Shell command output
- LLM prompts and responses
