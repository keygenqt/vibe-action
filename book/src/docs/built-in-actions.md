# Built-in Actions

Vibe Action embeds two actions in the binary: `docs` and `info`.

Additional commands ship in the
[vibe-action-groups](https://github.com/keygenqt/vibe-action-groups)
repository. The default config already connects it via action groups, so
these commands are available out of the box as `vibe-action <group> <action>`.

Groups are declared in `config.yaml` — you can add your own repositories
or local directories the same way. See [Configuration](./configuration.md)
for the `groups` block and the repository for its groups and sources.

## Quick Reference

| Action | About                            | Input          | Output   |
| ------ | -------------------------------- | -------------- | -------- |
| `docs` | Ask a question about Vibe Action | `query_prompt` | `dialog` |
| `info` | Show system information          | _(none)_       | `dialog` |

Query type semantics (`query_prompt`, ...) are described in
[Query Providers](./query-providers.md). Output targets (`replace`,
`clipboard`, `dialog`) — in [IDE Plugin](./ide-plugin.md).

## Auto-update

On first run, built-in actions are written to `~/.vibe-action/actions/`.
On every start, the engine compares the on-disk `version` field with
`PIPELINE_VERSION` (currently `0.0.2`). If they differ, the file is
overwritten with the embedded default. This keeps built-in actions in
sync with the engine — but it also means **manual edits to a built-in
file are lost when its version bumps**.

Grouped actions are never touched: only the embedded files
(`docs`, `info`) are subject to the version check.

Override the actions directory with `VIBE_ACTION_PATH` — see
[CLI Reference](./cli-reference.md).

## Customizing

To modify a built-in action, copy it to a new file with a different
`name`. The original will be reset on version bumps; your copy won't.

See [Custom Actions](./custom-actions.md) for writing pipelines from
scratch, and [Configuration](./configuration.md) for action groups.
