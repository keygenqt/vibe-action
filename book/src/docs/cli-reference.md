# CLI Reference

## Global Flags

| Flag              | Description                                                 |
| ----------------- | ----------------------------------------------------------- |
| `--config <path>` | Path to config file (default: `~/.vibe-action/config.yaml`) |
| `--debug`         | Enable verbose logging output                               |

## Commands

### `action`

Execute a YAML-defined action.

```bash
vibe-action action <name> [args...]
vibe-action action commit -p .
vibe-action action translate -f README.md -l Russian
vibe-action action --help
```

### `prompt`

Send a direct prompt to the LLM cluster, bypassing YAML actions.

```bash
vibe-action prompt <text...>
vibe-action prompt "Explain Rust lifetimes"
vibe-action prompt "Напиши функцию сортировки на Python"
```

## Action Arguments

Each action defines its own arguments in YAML. Use `--help` to see available options:

```bash
vibe-action action commit --help
vibe-action action extract --help
```

## Exit Codes

| Code | Description                                                |
| ---- | ---------------------------------------------------------- |
| 0    | Success                                                    |
| 1    | Error (validation failed, shell command failed, LLM error) |

## Debug Mode

Use `--debug` for detailed logs of each pipeline step:

```bash
vibe-action --debug action commit -p .
```

Shows:

- Original and resolved action text
- Shell command output
- LLM prompts and responses
