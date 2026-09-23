# IDE Integration (VS Code & IntelliJ)

Vibe Action works perfectly from the terminal, but you can supercharge your workflow with the Vibe Action Cross IDE plugin.

Instead of manually configuring `tasks.json` in VS Code or `External Tools` in IntelliJ, the plugin provides a native UI panel inside your editor. It communicates directly with the `vibe-action` CLI to discover all available actions, letting you run them with visible, real-time feedback.

## Why Use the Plugin?

- **Zero Configuration:** no need to edit JSON/XML files. The plugin automatically fetches all available actions from the `vibe-action` CLI — including built-in commands, your custom YAML flows, and any modifications you've made to the defaults.
- **Context Awareness:** automatically passes selected text, file paths, or project context to your actions using query tags.
- **Native UI:** real-time progress feedback inside the IDE instead of waiting for terminal windows or system notifications.
- **Seamless Output:** results can automatically replace selected code, be copied to the clipboard, or appear in a native dialog window.

## Prerequisites

Vibe Action CLI must be installed and available on your system `PATH`.

```text
cargo install vibe-action
```

Get the plugin: download the latest pre-built artifacts (`.vsix` and `.zip`) directly from the [dist directory on GitCode](https://gitcode.com/keygenqt_vz/vibe-action-cross/tree/main/dist), or build them yourself from the [vibe-action-cross](https://gitcode.com/keygenqt_vz/vibe-action-cross) repository source.

## VS Code Setup

1. Open VS Code.
2. Go to the Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`).
3. Click the `...` menu in the top-right corner of the Extensions panel.
4. Select **Install from VSIX...**.
5. Navigate to the downloaded file and select `vibe-action-<version>.vsix`.
6. Reload VS Code when prompted.

Alternatively, install via CLI:

```text
code --install-extension vibe-action-<version>.vsix
```

Once installed, open the Vibe Action panel from the activity bar. You will see a list of all your available actions. Click any action to run it, or use the provided keyboard shortcuts.

## IntelliJ IDEA Setup

1. Open IntelliJ IDEA.
2. Go to `Settings/Preferences` -> `Plugins`.
3. Click the gear icon (`⚙️`) in the top-right corner of the Plugins window.
4. Select **Install Plugin from Disk...**.
5. Navigate to the downloaded file and select `vibe-action-<version>.zip`.
6. Restart IntelliJ IDEA when prompted.

Once installed, open the Vibe Action tool window (usually located on the right sidebar). The UI is rendered natively using Compose Multiplatform and matches the IntelliJ theme.

## The `api` Block

The optional `api` block in a YAML action tells the plugin where to take input from and how to present the result. The CLI runtime ignores it.

```yaml
version: 0.0.2
name: upper
about: Convert text to UPPERCASE
api:
  output: replace # replace | clipboard | dialog
  input: query_raw # any query tag, e.g. query_raw, query_prompt, query_image
  args: # Optional: extra inputs beyond the main query
    arg_file: query_file_path
actions:
  - tag: tag_upper
    run: value
    val:
      - name: text
        data: query_raw
        mods: 'upper'
    action: '{text}'
```

### Output Targets (`api.output`)

- `replace` — the plugin replaces the currently selected text in your editor with the action's result.
- `clipboard` — the plugin copies the result to your system clipboard.
- `dialog` — the result is shown in a native IDE popup/dialog.

Note: prefer `output: clipboard` over the `clipboard_text` / `clipboard_image` operators in pipelines meant for the IDE — those operators are no-ops when the action is driven by the plugin.

### Input Sources (`api.input`)

The `input` field names the query tag the plugin fills before executing the CLI: `query_raw` (raw text), `query_prompt` (interactive dialog), `query_file_path`, `query_project_path`, `query_line`, `query_image`, and others. The full list with CLI and IDE behavior lives in [Query Providers](./query-providers.md).

If an action does not have an `api` block, the plugin simply executes it and falls back to standard CLI behavior.

### Extra Inputs (`api.args`)

`args` maps argument names to query tags. Use it when a flow needs
multiple inputs beyond the main query: each key is an argument name
(matching an `args` entry), each value is a query tag. The IDE collects
them and passes them to the CLI automatically.
