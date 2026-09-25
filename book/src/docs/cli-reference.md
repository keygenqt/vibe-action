# CLI Reference

## Usage

```text
vibe-action <name> [query] [args...]

vibe-action commit .
vibe-action translate README.md -l Russian
vibe-action tone          # reads from clipboard if no query provided
vibe-action --help
```

Each `<name>` maps to a YAML-defined action. Use `--help` to list all
available actions and their arguments:

```text
vibe-action --help
vibe-action commit --help
```

## System commands

```text
vibe-action status   # Show version, paths, action counts
vibe-action clean    # Remove stale cache, temp files, clipboard
vibe-action stop     # Stop all running vibe-action processes
```

## Action arguments

Actions define their own CLI arguments in YAML. See
[YAML Format](./pipeline-yaml.md) for the `args` schema.

```text
vibe-action commit -d        # dry-run flag (bool)
vibe-action extract -f log.txt  # file argument (string)
```

## Singleton guard

By default, only one `vibe-action` process runs at a time. Starting a
new action automatically stops the previous one (graceful shutdown with
a 3-second timeout, then force-kill). This keeps shared state
(cache, local LLMs) predictable.

Disable with `VIBE_SKIP_LOCK`:

```text
VIBE_SKIP_LOCK=1 vibe-action review main.rs
```

Useful for parallel API clusters or when running multiple actions
concurrently.

## Interactive confirm

Actions with `ask: true` prompt for user approval before execution:

- **CLI** — `inquire` confirm dialog with the command preview.
- **JSON mode** — confirm protocol via stdin/stdout (30-second timeout,
  fail-closed).

## Interactive prompt

Actions using `query_prompt` trigger an interactive input when no query
is provided on the command line:

- **CLI** — `inquire` text prompt labeled "Query".
- **IDE** — plugin opens an input dialog.

## Desktop notifications

Actions with `notify: true` send a desktop notification on completion
(CLI mode only):

- **macOS** — via `terminal-notifier` (install: `brew install terminal-notifier`).
- **Linux** — via `notify-rust` (freedesktop.org D-Bus spec).

## Role mismatch warning

If a pipeline uses a `run` size (e.g. `large`) but no cluster node has
the matching `role`, the engine warns and offers to continue with all
available nodes (fallback). In CLI mode this is a confirm dialog; in
JSON mode it uses the confirm protocol.

See [Configuration](./configuration.md) for cluster role setup.

## Environment variables

| Variable           | Description                                    | Default                      |
| ------------------ | ---------------------------------------------- | ---------------------------- |
| `VIBE_CONFIG`      | Path to config file                            | `~/.vibe-action/config.yaml` |
| `VIBE_ACTION_PATH` | Path to actions directory                      | `~/.vibe-action/actions/`    |
| `VIBE_SKIP_LOCK`   | Disable singleton guard for parallel execution | Not set (guard enabled)      |
| `VIBE_TEST`        | Internal test mode (`1` = enabled)             | Not set                      |

Output-related variables (`VIBE_LOG_TYPE`, `VIBE_TRACE_LEVEL`) and trace
levels — see [Output Modes](./output-modes.md).

## Exit codes

| Code  | Description                                         |
| ----- | --------------------------------------------------- |
| `0`   | Success                                             |
| `1`   | Error (validation, shell command, LLM failure)      |
| `130` | Instance superseded by a newer run (auto-cancelled) |
