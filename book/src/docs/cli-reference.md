# CLI Reference

## Environment Variables

| Variable           | Description                       | Default                      |
| ------------------ | --------------------------------- | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file               | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory         | `~/.vibe-action/actions/`    |
| `VIBE_DEBUG`       | Enable debug mode (`1` or `true`) | disabled                     |
| `VIBE_LOG_LEVEL`   | Tracing level (1-6)               | `4` (debug)                  |

## Log Levels

| Level | Name  | Description                |
| ----- | ----- | -------------------------- |
| 1     | error | Errors only                |
| 2     | warn  | Warnings and errors        |
| 3     | info  | Flow progress and results  |
| 4     | debug | Detailed engine internals  |
| 5     | trace | Maximum verbosity          |
| 6     | test  | Test mode (minimal output) |

## Commands

Execute a YAML-defined action directly:

```bash
vibe-action <name> [args...]
vibe-action commit -p .
vibe-action translate -f README.md -l Russian
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

Set `VIBE_DEBUG=1` for detailed logs of each pipeline step:

```bash
VIBE_DEBUG=1 vibe-action commit -p .
```

Shows:

- Original and resolved action text
- Shell command output
- LLM prompts and responses
