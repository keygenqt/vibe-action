# IDE Integration (VS Code & IntelliJ)

Vibe Action works perfectly from the terminal, but you can supercharge your workflow with the **Vibe Action Cross** IDE plugin.

Instead of manually configuring `tasks.json` in VS Code or `External Tools` in IntelliJ, the plugin provides a native UI panel inside your editor. It communicates directly with the `vibe-action` CLI to discover all available actions, letting you run them with visible, real-time feedback.

## Why Use the Plugin?

- **Zero Configuration:** No need to edit JSON/XML files. The plugin automatically fetches all available actions from the `vibe-action` CLI — including built-in commands, your custom YAML flows, and any modifications you've made to the defaults.
- **Context Awareness:** Automatically passes selected text or clipboard content to your actions using the `api` block defined in your YAML manifests.
- **Native UI:** Get real-time progress feedback inside the IDE instead of waiting for terminal windows or system notifications.
- **Seamless Output:** Results can automatically replace selected code, be copied to the clipboard, or appear in a native dialog window.

## Prerequisites

1. **Vibe Action CLI** must be installed and available on your system `PATH`.
   ```bash
   cargo install vibe-action
   ```
2. Get the plugin: Download the latest pre-built artifacts (`.vsix` and `.zip`) directly from the [dist directory on GitCode](https://gitcode.com/keygenqt_vz/vibe-action-cross/tree/main/dist), or build them yourself from the [vibe-action-cross](https://gitcode.com/keygenqt_vz/vibe-action-cross) repository source.

## VS Code Setup

1. Open VS Code.
2. Go to the Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`).
3. Click the `...` menu in the top-right corner of the Extensions panel.
4. Select **Install from VSIX...**.
5. Navigate to the downloaded file and select `vibe-action-0.0.1.vsix`.
6. Reload VS Code when prompted.

_Alternatively, install via CLI:_

```bash
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

## How It Works: The `api` Block

The `vibe-action` CLI exposes the optional `api` block from your YAML actions, which the plugin uses to determine how to handle inputs and outputs. If you are creating custom actions or modifying built-in ones, you can define this block to make them IDE-friendly.

```yaml
name: upper
about: Convert text to UPPERCASE
api:
  output: replace # How to output: replace | clipboard | dialog
  args:
    text: selection # Where to get input: selection | clipboard
args:
  - name: text
    short: t
    input: string
    default: '{system_clipboard}'
actions:
  - tag: tag_upper
    run: value
    expect: string
    action: '{text|upper}'
```

### Output Targets

- `replace`: The plugin will replace the currently selected text in your editor with the action's result.
- `clipboard`: The result will be silently copied to your system clipboard.
- `dialog`: The result will be shown in a native IDE popup/dialog.

### Input Contexts

- `selection`: The plugin will automatically grab the text you currently have highlighted in the editor and pass it to the argument.
- `clipboard`: The plugin will pass the contents of your system clipboard to the argument.

If an action does not have an `api` block, the plugin will simply execute it and fall back to standard CLI behavior (usually copying to clipboard).
