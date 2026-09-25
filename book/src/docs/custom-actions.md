# Custom Actions

Add your own YAML files to `~/.vibe-action/actions/` — they are loaded
automatically alongside the built-in defaults. The file name (without
`.yaml`) becomes the CLI subcommand unless `name` overrides it.

## Minimal action

```yaml
version: '0.0.2'
name: hello
about: Say hello

actions:
  - tag: tag_greeting
    run: value
    val:
      - name: input
        data: query_raw
    action: 'Hello, {input}!'
```

```text
vibe-action hello World
# → Hello, World!
```

## File-path / text dual mode

A common pattern: if the input is a valid path, read the file; otherwise
treat it as inline text.

```yaml
actions:
  - tag: tag_content
    run: value
    reg: '.+'
    val:
      - name: content
        data: query_raw
        when: 'is:path'
        mods: 'fetch|text'
      - name: content
        data: query_raw
        fail: 'is:empty:not'
    action: '{content}'
```

## CLI arguments

Define flags; each is available in val candidates as `data: <name>`
(convention: prefix names with `arg_`):

```yaml
args:
  - name: arg_style
    short: 's'
    input: string
    default: concise
    help: 'Output style'
```

```text
vibe-action my-action --arg_style verbose
```

Argument values are accessed as `data: arg_style` in val candidates.

## LLM prompt with system tags

Use `system_*` providers to adapt prompts to the user's environment:

```yaml
actions:
  - tag: tag_result
    run: medium
    val:
      - name: lang
        data: system_language
      - name: content
        data: query_raw
    action: |
      [Task]
      Respond in {lang}.
      {content}
```

## Multi-step pipeline

Actions can chain results via `data` referencing other action tags. The
engine resolves dependencies automatically:

```yaml
actions:
  - tag: tag_path
    run: value
    val:
      - name: dir
        data: system_dir_download
      - name: pid
        data: system_pid
    action: '{dir}/output-{pid}.txt'

  - tag: tag_save
    run: value
    val:
      - name: content
        data: query_raw
        mods: 'file:{tag_path}'
    action: 'Saved to {tag_path}'
```

`tag_save` depends on `tag_path` — the engine runs `tag_path` first.

## Shell commands

`run: cmd` executes via `sh -c`. Values are shell-quoted automatically:

```yaml
actions:
  - tag: tag_files
    run: cmd
    val:
      - name: path
        data: query_project_path
    action: find {path} -type f -name '*.rs'
```

## Fan-out with each

Process each item in a list independently, then merge results:

```yaml
actions:
  - tag: tag_results
    run: small
    val:
      - name: item
        data: tag_items
        each:
          split: '\n'
          merge: '\n'
    action: |
      [Task]
      Summarize: {item}
```

## IDE integration

Add an `api` block so the IDE plugin knows how to handle the action:

```yaml
api:
  output: replace
  input: query_raw
  args:
    arg_file: query_file_path
```

See [IDE Plugin](./ide-plugin.md) for the full API reference.

## Customizing built-in actions

Do not edit built-in files directly — they are reset on version bumps.
See [Built-in Actions](./built-in-actions.md).

## Validation

All custom actions are validated on startup. Common errors:

| Error                                 | Fix                                         |
| ------------------------------------- | ------------------------------------------- |
| Version mismatch                      | Set `version` to current `PIPELINE_VERSION` |
| Empty `name` or `about`               | Add required fields                         |
| Duplicate `tag` across actions        | Use unique tag names                        |
| `data` references own `tag`           | Remove self-reference                       |
| Bare `query` in `data`                | Use `query_raw` instead                     |
| `query_*`/`system_*` prefix on tag    | Rename the tag                              |
| Unknown operator in `mods`            | Check operator name and spelling            |
| Non-inspect operator in `when`/`fail` | Use inspect operators only                  |
| Undeclared `{name}` in `action`       | Add a val candidate with that `name`        |

## Tips

- **Start simple** — a single `run: value` or `run: small` action is
  enough for most use cases.
- **Use `reg`** — validate output format early. A loose LLM can produce
  unexpected structure; `reg: '.+'` catches empty results.
- **Use `fail` guards** — catch empty inputs before they reach the LLM:
  `fail: 'is:empty:not'`.
- **Use `when` guards** — avoid unnecessary work (e.g. don't take a
  screenshot if the clipboard already has an image).
- **Use `ask: true`** — for destructive shell commands (git push, file
  deletion), require user confirmation.
- **Test with `--help`** — your action and its args appear in the help
  output automatically.
