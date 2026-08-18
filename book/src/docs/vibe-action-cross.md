# IDE Integration (VS Code & IntelliJ)

Vibe Action works perfectly from the terminal, but you can supercharge your workflow with the Vibe Action Cross IDE plugin.

Instead of manually configuring `tasks.json` in VS Code or `External Tools` in IntelliJ, the plugin provides a native UI panel inside your editor. It communicates directly with the `vibe-action` CLI to discover all available actions, letting you run them with visible, real-time feedback.

## Why Use the Plugin?

- **Zero Configuration:** no need to edit JSON/XML files. The plugin automatically fetches all available actions from the `vibe-action` CLI — including built-in commands, your custom YAML flows, and any modifications you've made to the defaults.
- **Context Awareness:** automatically passes selected text, file paths, or project context to your actions using the unified `{query}` tag.
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
5. Navigate to the downloaded file and select `vibe-action-0.0.1.vsix`.
6. Reload VS Code when prompted.

Alternatively, install via CLI:

```text
code --install-extension vibe-action-0.0.1.vsix
```

Once installed, open the Vibe Action panel from the activity bar. You will see a list of all your available actions. Click any action to run it, or use the provided keyboard shortcuts.

## IntelliJ IDEA Setup

1. Open IntelliJ IDEA.
2. Go to `Settings/Preferences` -> `Plugins`.
3. Click the gear icon (`⚙️`) in the top-right corner of the Plugins window.
4. Select **Install Plugin from Disk...**.
5. Navigate to the downloaded file and select `vibe-action-plugin-0.0.1.zip`.
6. Restart IntelliJ IDEA when prompted.

Once installed, open the Vibe Action tool window (usually located on the right sidebar). The UI is rendered natively using Compose Multiplatform and matches the IntelliJ theme.

## The `api` Block

The optional `api` block in a YAML action tells the plugin where to take input from and how to present the result:

```yaml
version: 0.0.1
name: upper
about: Convert text to UPPERCASE
api:
  output: replace # How to output: replace | clipboard | dialog
  input: query # Where to get input: any query type
  args: # Optional: extra inputs beyond {query}
    file: query|file_path
actions:
  - tag: tag_upper
    run: value
    expect: string
    action: '{query|upper}'
```

### Output Targets (`api.output`)

- `replace` — the plugin replaces the currently selected text in your editor with the action's result.
- `clipboard` — the result is silently copied to your system clipboard.
- `dialog` — the result is shown in a native IDE popup/dialog.

### Input Sources (`api.input`)

The `input` field tells the plugin how to fill the `{query}` tag before executing the CLI. It accepts any query type — `query`, `query|file_path`, `query|project_path`, `query|line`, `query|prompt`, `query|image`. The full table of types with CLI and IDE behavior lives in [Query Tag](./query-tag.md).

If an action does not have an `api` block, the plugin simply executes it and falls back to standard CLI behavior.

### Extra Inputs (`api.args`)

`args` maps additional argument names to query types. Use it when a flow needs multiple inputs beyond the main `{query}` positional arg: each key is an argument name, each value is a query type (e.g., `query|file_path`). The IDE collects them and passes to the CLI automatically.
